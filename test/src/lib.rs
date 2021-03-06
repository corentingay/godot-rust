use gdnative::*;

mod test_derive;
mod test_free_ub;
mod test_register;
mod test_return_leak;
mod test_variant_call_args;

#[no_mangle]
pub extern "C" fn run_tests(
    _data: *mut gdnative::libc::c_void,
    _args: *mut gdnative::sys::godot_array,
) -> gdnative::sys::godot_variant {
    let mut status = true;
    status &= gdnative::test_string();

    status &= gdnative::test_dictionary();
    // status &= gdnative::test_dictionary_clone_clear();

    status &= gdnative::test_array();
    // status &= gdnative::test_array_clone_clear();

    status &= gdnative::test_variant_nil();
    status &= gdnative::test_variant_i64();
    status &= gdnative::test_variant_bool();

    status &= gdnative::test_vector2_variants();

    status &= gdnative::test_vector3_variants();

    status &= gdnative::test_variant_option();
    status &= gdnative::test_variant_result();
    status &= gdnative::test_to_variant_iter();
    status &= gdnative::test_variant_tuple();

    status &= gdnative::test_byte_array_access();
    status &= gdnative::test_int32_array_access();
    status &= gdnative::test_float32_array_access();
    status &= gdnative::test_color_array_access();
    status &= gdnative::test_string_array_access();
    status &= gdnative::test_vector2_array_access();
    status &= gdnative::test_vector3_array_access();

    status &= test_constructor();
    status &= test_underscore_method_binding();
    status &= test_rust_class_construction();

    status &= test_derive::run_tests();
    status &= test_free_ub::run_tests();
    status &= test_register::run_tests();
    status &= test_return_leak::run_tests();
    status &= test_variant_call_args::run_tests();

    gdnative::Variant::from_bool(status).forget()
}

fn test_constructor() -> bool {
    println!(" -- test_constructor");

    // Just create an object and call a method as a sanity check for the
    // generated constructors.
    let lib = GDNativeLibrary::new();
    let _ = lib.is_singleton();

    unsafe {
        let path = FreeOnDrop::new(Path2D::new());
        let _ = path.get_z_index();
    }

    return true;
}

fn test_underscore_method_binding() -> bool {
    println!(" -- test_underscore_method_binding");

    let ok = std::panic::catch_unwind(|| {
        let table = gdnative::NativeScriptMethodTable::get(get_api());
        assert_ne!(0, table._new as usize);
    })
    .is_ok();

    if !ok {
        godot_error!("   !! Test test_underscore_method_binding failed");
    }

    ok
}

struct Foo(i64);

impl NativeClass for Foo {
    type Base = Reference;
    type UserData = user_data::ArcData<Foo>;
    fn class_name() -> &'static str {
        "Foo"
    }
    fn init(_owner: Reference) -> Foo {
        Foo(42)
    }
    fn register_properties(_builder: &init::ClassBuilder<Self>) {}
}

struct NotFoo;

impl NativeClass for NotFoo {
    type Base = Reference;
    type UserData = user_data::ArcData<NotFoo>;
    fn class_name() -> &'static str {
        "NotFoo"
    }
    fn init(_owner: Reference) -> NotFoo {
        NotFoo
    }
    fn register_properties(_builder: &init::ClassBuilder<Self>) {}
}

#[methods]
impl Foo {
    #[export]
    fn answer(&self, _owner: Reference) -> i64 {
        self.0
    }

    #[export]
    fn choose(
        &self,
        _owner: Reference,
        a: GodotString,
        which: bool,
        b: GodotString,
    ) -> GodotString {
        if which {
            a
        } else {
            b
        }
    }

    #[export]
    fn choose_variant(&self, _owner: Reference, a: i32, what: Variant, b: f64) -> Variant {
        let what = what.try_to_string().expect("should be string");
        match what.as_str() {
            "int" => a.to_variant(),
            "float" => b.to_variant(),
            _ => panic!("should be int or float, got {:?}", what),
        }
    }
}

fn test_rust_class_construction() -> bool {
    println!(" -- test_rust_class_construction");

    let ok = std::panic::catch_unwind(|| {
        let foo = Instance::<Foo>::new();

        assert_eq!(Ok(42), foo.map(|foo, owner| { foo.answer(owner) }));

        let mut base = foo.into_base();
        assert_eq!(
            Some(42),
            unsafe { base.call("answer".into(), &[]) }.try_to_i64()
        );

        let foo = Instance::<Foo>::try_from_base(base).expect("should be able to downcast");
        assert_eq!(Ok(42), foo.map(|foo, owner| { foo.answer(owner) }));

        let base = foo.into_base();
        assert!(Instance::<NotFoo>::try_from_base(base).is_none());
    })
    .is_ok();

    if !ok {
        godot_error!("   !! Test test_rust_class_construction failed");
    }

    ok
}

fn init(handle: init::InitHandle) {
    handle.add_class::<Foo>();

    test_derive::register(&handle);
    test_free_ub::register(&handle);
    test_register::register(&handle);
    test_return_leak::register(&handle);
    test_variant_call_args::register(&handle);
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
