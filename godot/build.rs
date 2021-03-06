
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::env;
use std::path::PathBuf;
use std::io::Write;
use std::fmt;
use std::borrow::Cow;

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
    type Reference = {name};

    fn godot_name() -> &'static str {{
        "{name}"
    }}

    unsafe fn register_class(_desc: *mut libc::c_void) {{
        panic!("Can't register");
    }}

    fn godot_info(&self) -> &GodotClassInfo {{
        &self.info
    }}
    unsafe fn reference(_this: *mut sys::godot_object, data: &Self::ClassData) -> &Self::Reference {{
        data
    }}
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

        if class.singleton {
            let s_name = if class.name.starts_with("_") {
                &class.name[1..]
            } else {
                class.name.as_ref()
            };
            writeln!(output, r#"
    pub fn godot_singleton() -> GodotRef<{name}> {{
        unsafe {{
            let obj = (get_api().godot_global_get_singleton)(b"{s_name}\0".as_ptr() as *mut _);
            GodotRef::from_raw(obj as *mut _)
        }}
    }}
            "#, name = class.name, s_name = s_name).unwrap();
        }

        'method:
        for method in class.methods {
            let rust_ret_type = if let Some(ty) = godot_type_to_rust(&method.return_type) {
                ty
            } else {
                continue
            };

            let mut type_params = String::new();
            let mut params = String::new();
            for (idx, argument) in method.arguments.iter().enumerate() {
                if let Some(ty) = godot_type_to_rust(&argument.ty) {
                    match argument.ty.as_str() {
                        "String" => {
                            let param = format!("P{}", idx);
                            fmt::Write::write_fmt(&mut type_params, format_args!("{}: AsRef<str>,", param)).unwrap();
                            fmt::Write::write_fmt(&mut params, format_args!(", {}: {}", rust_safe_name(&argument.name), param)).unwrap();
                        },
                        _ => {
                            fmt::Write::write_fmt(&mut params, format_args!(", {}: {}", rust_safe_name(&argument.name), ty)).unwrap();
                        },
                    }
                } else {
                    continue 'method;
                }
            }

            if method.has_varargs {
                params.push_str(", varargs: &[Variant]");
            }

            writeln!(output, r#"

    pub fn {name}<{type_params}>(&self{params}) -> {rust_ret_type} {{
        use std::ptr;
        unsafe {{
            let api = ::get_api();
            static mut METHOD_BIND: *mut sys::godot_method_bind = 0 as _;
            static INIT: Once = ONCE_INIT;
            INIT.call_once(|| {{
                let class = b"{cname}\0".as_ptr();
                let method = b"{name}\0".as_ptr();
                METHOD_BIND = (api.godot_method_bind_get_method)(
                    class as *const _,
                    method as *const _
                );
                debug_assert!(!METHOD_BIND.is_null());
            }});

            "#, cname = class.name, name = method.name, rust_ret_type = rust_ret_type, params = params,
                type_params = type_params).unwrap();
            if method.has_varargs {
                writeln!(output, r#"
            let mut argument_buffer: Vec<*const sys::godot_variant> = Vec::with_capacity({arg_count} + varargs.len());
                "#, arg_count = method.arguments.len()).unwrap();

                for argument in &method.arguments {
                    let ty = godot_type_to_rust(&argument.ty).unwrap();
                    if ty.starts_with("Option") {
                        writeln!(output, r#"
                let {name}: Variant = if let Some(o) = {name} {{
                    o.into()
                }} else {{ Variant::new_nil() }};
                        "#, name = rust_safe_name(&argument.name)).unwrap();
                    } else if ty == "String" {
                        writeln!(output, r#"
                let {name}: Variant = Variant::new_string({name});
                        "#, name = rust_safe_name(&argument.name)).unwrap();
                    } else {
                        writeln!(output, r#"
                let {name}: Variant = {name}.into();
                        "#, name = rust_safe_name(&argument.name)).unwrap();
                    }
                    writeln!(output, r#"
            argument_buffer.push(&{name}.0);
                    "#, name = rust_safe_name(&argument.name)).unwrap();
                }

                writeln!(output, r#"
            for arg in varargs {{
                argument_buffer.push(&arg.0 as *const _);
            }}
            let ret = Variant((api.godot_method_bind_call)(METHOD_BIND, self.info.this, argument_buffer.as_mut_ptr(), argument_buffer.len() as _, ptr::null_mut()));
                "#).unwrap();

                if rust_ret_type.starts_with("Option") {
                    writeln!(output, r#"
                ret.as_object()
                    "#).unwrap();
                } else {
                    writeln!(output, r#"
                ret.into()
                    "#).unwrap();
                }

            } else {
                writeln!(output, r#"
            let mut argument_buffer = [ptr::null() as *const libc::c_void; {arg_count}];
                "#, arg_count = method.arguments.len()).unwrap();

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
            }

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
        "in" => "_in",
        "override" => "_override",
        "where" => "_where",
        name => name,
    }
}

fn godot_type_to_rust(ty: &str) -> Option<Cow<str>> {
    match ty {
        "void" => Some("()".into()),
        "String" => Some("String".into()),
        "float" => Some("f64".into()),
        "int" => Some("i64".into()),
        "bool" => Some("bool".into()),
        "Vector2" => Some("Vector2".into()),
        "Vector3" => Some("Vector3".into()),
        "Basis" => Some("Basis".into()),
        "Color" => Some("Color".into()),
        "NodePath" => Some("NodePath".into()),
        "Variant" => Some("Variant".into()),
        "Array" => None, // TODO:
        "AABB" => None, // TODO:
        "RID" => None, // TODO:
        "Rect2" => None, // TODO:
        "Rect3" => None, // TODO:
        "Plane" => None, // TODO:
        "Quat" => None, // TODO:
        "Transform" => None, // TODO:
        "Transform2D" => None, // TODO:
        "Dictionary" => None, // TODO:
        "PoolStringArray" => None, // TODO:
        "PoolByteArray" => None, // TODO:
        "PoolVector2Array" => None, // TODO:
        "PoolVector3Array" => None, // TODO:
        "PoolIntArray" => None, // TODO:
        "PoolRealArray" => None, // TODO:
        "PoolColorArray" => None, // TODO:
        ty if ty.starts_with("enum.") => None, // TODO: Enums
        ty => {
            Some(format!("Option<GodotRef<{}>>", ty).into())
        },
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
        "int" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "String" => {
            writeln!(w, r#"
            let mut __val_{arg} = {name}.as_ref();
            let mut __arg_{arg} = (api.godot_string_chars_to_utf8_with_len)(__val_{arg}.as_ptr() as *const _, __val_{arg}.len() as _);
            argument_buffer[{arg}] = (&__arg_{arg}) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "Vector2" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}.0) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "Vector3" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}.0) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "Basis" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}.0) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "Color" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}.0) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "NodePath" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}.0) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        "Variant" => {
            writeln!(w, r#"
            argument_buffer[{arg}] = (&{name}.0) as *const _ as *const _;
            "#, name = name, arg = arg).unwrap();
        },
        _ty => {
            writeln!(w, r#"
            argument_buffer[{arg}] = if let Some(arg) = {name} {{
                arg.this as *const _ as *const _
            }} else {{
                ptr::null()
            }};
            "#, name = name, arg = arg).unwrap();
        },
    }
}
fn godot_handle_argument_post<W: Write>(w: &mut W, ty: &str, arg: usize) {
    match ty {
        "bool" => {},
        "float" => {},
        "int" => {},
        "Vector2" => {},
        "Vector3" => {},
        "Basis" => {},
        "Color" => {},
        "NodePath" => {},
        "Variant" => {},
        "String" => {
            writeln!(w, r#"
            (api.godot_string_destroy)(&mut __arg_{arg});
            "#, arg = arg).unwrap();
        }
        _ty => {},
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
        "int" => {
            writeln!(w, r#"
            let mut ret = 0i64;
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
        "Vector2" => {
            writeln!(w, r#"
            let mut ret = sys::godot_vector2::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "Vector3" => {
            writeln!(w, r#"
            let mut ret = sys::godot_vector3::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "Basis" => {
            writeln!(w, r#"
            let mut ret = sys::godot_basis::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "Color" => {
            writeln!(w, r#"
            let mut ret = sys::godot_color::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "NodePath" => {
            writeln!(w, r#"
            let mut ret = sys::godot_node_path::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        "Variant" => {
            writeln!(w, r#"
            let mut ret = sys::godot_variant::default();
            let ret_ptr = &mut ret as *mut _;
            "#).unwrap();
        },
        _ty => {
            writeln!(w, r#"
            let mut ret: *mut sys::godot_object = ptr::null_mut();
            let ret_ptr = (&mut ret) as *mut _;
            "#).unwrap();
        }
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
        "int" => {
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
            let __tmp = (api.godot_string_utf8)(&ret);
            ::std::ffi::CStr::from_ptr((api.godot_char_string_get_data)(&__tmp) as *const _)
                .to_string_lossy()
                .into_owned()
            "#).unwrap();
        },
        "Vector2" => {
            writeln!(w, r#"
            Vector2(ret)
            "#).unwrap();
        },
        "Vector3" => {
            writeln!(w, r#"
            Vector3(ret)
            "#).unwrap();
        },
        "Basis" => {
            writeln!(w, r#"
            Basis(ret)
            "#).unwrap();
        },
        "Color" => {
            writeln!(w, r#"
            Color(ret)
            "#).unwrap();
        },
        "NodePath" => {
            writeln!(w, r#"
            NodePath(ret)
            "#).unwrap();
        },
        "Variant" => {
            writeln!(w, r#"
            Variant(ret)
            "#).unwrap();
        },
        ty => {
            writeln!(w, r#"
            if ret.is_null() {{
                None
            }} else {{
                Some(GodotRef::<{}>::from_object(ret))
            }}
            "#, ty).unwrap();
        },
    }
}

#[derive(Deserialize, Debug)]
struct GodotClass {
    name: String,
    base_class: String,
    singleton: bool,
    is_reference: bool,

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