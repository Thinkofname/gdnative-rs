
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::env;
use std::path::PathBuf;
use std::io::Write;
use std::fmt;

fn main() {
    let classes: Vec<GodotClass> = serde_json::from_reader(File::open("api.json").unwrap())
        .expect("Failed to parse api.json");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut output = File::create(out_path.join("types.rs")).unwrap();

    for class in classes {
        writeln!(output, r#"
#[allow(non_camel_case_types)]
pub struct {name} {{
    info: GodotClassInfo,
"#, name = class.name).unwrap();
        if class.base_class != "" {
            writeln!(output, r#"
    parent: {parent},
            "#, parent = class.base_class).unwrap();
        }
        writeln!(output, r#"
}}

unsafe impl GodotClass for {name} {{
    type ClassData = {name};

    fn godot_name() -> &'static str {{
        "{name}"
    }}

    unsafe fn register_class(_desc: *mut libc::c_void) {{
        panic!("Can't register");
    }}

    fn godot_info(&self) -> &GodotClassInfo {{
        &self.info
    }}
}}
unsafe impl GodotNativeClass for {name} {{
    unsafe fn from_object(obj: *mut sys::godot_object) -> Self {{
        {name} {{
            info: GodotClassInfo {{
                this: obj,
            }},
"#, name = class.name).unwrap();
        if class.base_class != "" {
            writeln!(output, r#"
            parent: {parent}::from_object(obj),
            "#, parent = class.base_class).unwrap();
        }
        writeln!(output, r#"
        }}
    }}
}}
"#).unwrap();
        if class.base_class != "" {
            writeln!(output, r#"
impl Deref for {name} {{
    type Target = {parent};
    fn deref(&self) -> &Self::Target {{
        &self.parent
    }}
}}
            "#, name = class.name, parent = class.base_class).unwrap();
        }
        writeln!(output, r#"
impl {name} {{
"#, name = class.name).unwrap();

        'method:
        for method in class.methods {
            let rust_ret_type = if let Some(ty) = godot_type_to_rust(&method.return_type) {
                ty
            } else {
                continue
            };

            let mut params = String::new();
            for argument in &method.arguments {
                let ty = if let Some(ty) = godot_type_to_rust(&argument.ty) {
                    ty
                } else {
                    continue 'method;
                };
                fmt::Write::write_fmt(&mut params, format_args!(", {}: {}", rust_safe_name(&argument.name), ty)).unwrap();
            }

            writeln!(output, r#"

    pub fn {name}(&self{params}) -> {rust_ret_type} {{
        use std::ptr;
        use std::ffi;
        unsafe {{
            let api = ::get_api();
            static mut METHOD_BIND: *mut sys::godot_method_bind = 0 as _;
            static INIT: Once = ONCE_INIT;
            INIT.call_once(|| {{
                let class = ffi::CString::new("{cname}").unwrap();
                let method = ffi::CString::new("{name}").unwrap();
                METHOD_BIND = (api.godot_method_bind_get_method)(
                    class.as_ptr() as *const _,
                    method.as_ptr() as *const _
                );
            }});

            let mut argument_buffer = [ptr::null() as *const libc::c_void; {arg_count}];
            "#, cname = class.name, name = method.name, rust_ret_type = rust_ret_type, params = params,
                arg_count = method.arguments.len()).unwrap();

            for (idx, argument) in method.arguments.iter().enumerate() {
                godot_handle_argument_pre(&mut output, &argument.ty, rust_safe_name(&argument.name), idx);
            }

            godot_handle_return_pre(&mut output, &method.return_type);

            writeln!(output, r#"
            (api.godot_method_bind_ptrcall)(METHOD_BIND, self.info.this, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
            "#).unwrap();

            for (idx, argument) in method.arguments.iter().enumerate() {
                godot_handle_argument_post(&mut output, &argument.ty, idx);
            }

            godot_handle_return_post(&mut output, &method.return_type);

            writeln!(output, r#"
        }}
    }}"#).unwrap();
        }

        writeln!(output, r#"}}"#).unwrap();
    }
}

fn rust_safe_name(name: &str) -> &str {
    match name {
        "use" => "_use",
        "type" => "_type",
        "loop" => "_loop",
        name => name,
    }
}

fn godot_type_to_rust(ty: &str) -> Option<&str> {
    match ty {
        "void" => Some("()"),
        "String" => Some("String"),
        "float" => Some("f64"),
        "bool" => Some("bool"),
        _ => None,
    }
}
fn godot_handle_argument_pre<W: Write>(w: &mut W, ty: &str, name: &str, arg: usize) {
    match ty {
        "bool" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "float" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "String" => {
            writeln!(w, r#"
            let mut __arg_{arg} = sys::godot_string::default();
            (api.godot_string_new_data)(&mut __arg_{arg}, {name}.as_ptr() as *const _, {name}.len() as _);
            argument_buffer[{arg}] = (&__arg_{arg}) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        _ => unimplemented!(),
    }
}
fn godot_handle_argument_post<W: Write>(w: &mut W, ty: &str, arg: usize) {
    match ty {
        "bool" => {},
        "float" => {},
        "String" => {
            writeln!(w, r#"
            let mut __arg_{arg} = sys::godot_string::default();
            (api.godot_string_destroy)(&mut __arg_{arg});
            "#, arg = arg).unwrap();
        }
        _ => unimplemented!(),
    }
}

fn godot_handle_return_pre<W: Write>(w: &mut W, ty: &str) {
    match ty {
        "void" => {
            writeln!(w, r#"
            let ret_ptr = ptr::null_mut();
            "#).unwrap();

        },
        "float" => {
            writeln!(w, r#"
            let mut ret = 0.0f64;
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "bool" => {
            writeln!(w, r#"
            let mut ret = false;
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "String" => {
            writeln!(w, r#"
            let mut ret = sys::godot_string::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        _ => unimplemented!(),
    }
}

fn godot_handle_return_post<W: Write>(w: &mut W, ty: &str) {
    match ty {
        "void" => {
        },
        "float" => {
            writeln!(w, r#"
            ret
            "#).unwrap();
        }
        "bool" => {
            writeln!(w, r#"
            ret
            "#).unwrap();
        }
        "String" => {
            writeln!(w, r#"
            ::std::ffi::CStr::from_ptr((api.godot_string_c_str)(&ret) as *const _)
                .to_string_lossy()
                .into_owned()
            "#).unwrap();
        },
        _ => unimplemented!(),
    }
}

#[derive(Deserialize, Debug)]
struct GodotClass {
    name: String,
    base_class: String,

    methods: Vec<GodotMethod>,
}

#[derive(Deserialize, Debug)]
struct GodotMethod {
    name: String,
    return_type: String,

    is_editor: bool,
    is_noscript: bool,
    is_const: bool,
    is_reverse: bool,
    is_virtual: bool,
    has_varargs: bool,

    arguments: Vec<GodotArgument>,
}

#[derive(Deserialize, Debug)]
struct GodotArgument {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    has_default_value: bool,
    default_value: String,

}