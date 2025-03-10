#![allow(unused)]

mod issue_261 {
    use py_rs::PY;

    trait Driver {
        type Info;
    }

    struct TsDriver;
    impl Driver for TsDriver {
        type Info = String;
    }

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/issue_261/")]
    struct OtherInfo {
        x: i32,
    }

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/issue_261/")]
    struct OtherDriver;
    impl Driver for OtherDriver {
        type Info = OtherInfo;
    }

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/issue_261/", concrete(T = TsDriver))]
    struct Consumer1<T: Driver> {
        info: T::Info,
    }

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/issue_261/", concrete(T = OtherDriver))]
    struct Consumer2<T: Driver> {
        info: T::Info,
        driver: T,
    }

    #[test]
    fn concrete_generic_param() {
        assert_eq!(
            Consumer1::<TsDriver>::decl(),
            "type Consumer1 = { info: string, };"
        );
        // `decl` must use the concrete generic, no matter what we pass in
        assert_eq!(
            Consumer1::<TsDriver>::decl(),
            Consumer1::<OtherDriver>::decl()
        );

        assert_eq!(
            Consumer2::<OtherDriver>::decl_concrete(),
            "type Consumer2 = { info: OtherInfo, driver: OtherDriver, };"
        );
    }
}

mod simple {
    use py_rs::PY;

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/simple/")]
    #[py(concrete(T = i32))]
    struct Simple<T> {
        t: T,
    }

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/simple/")]
    struct Tuple<T> {
        f: Option<T>,
    }

    #[derive(PY)]
    #[py(export, export_to = "concrete_generic/simple/")]
    #[py(concrete(T = i32))]
    struct WithOption<T> {
        opt: Option<T>,
    }

    #[test]
    fn simple() {
        assert_eq!(Simple::<String>::decl(), "type Simple = { t: number, };");
        assert_eq!(
            WithOption::<String>::decl(),
            "type WithOption = { opt: number | null, };"
        );
    }
}
