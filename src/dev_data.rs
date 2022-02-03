use crate::state::{BindTrace, DebuggerState, ExprTrace, FnCallTrace, Form};
use std::sync::{Arc, Mutex};

pub fn add_nested_let_flow(debugger_state_arc: &Arc<Mutex<DebuggerState>>) {
    let mut state = debugger_state_arc
        .lock()
        .expect("Can't get the lock on state mutex");
}

pub fn add_factorial(debugger_state_arc: &Arc<Mutex<DebuggerState>>) {
    let mut state = debugger_state_arc
        .lock()
        .expect("Can't get the lock on state mutex");

    state.add_flow_form(
        1709,
        71712880,
        Form::new(
            "(defn factorial [n] (if (zero? n) 1 (* n (factorial (dec n)))))".to_string(),
            1643740412526,
        ),
        1643740412526,
    );
    state.add_fn_call_trace(
        1709,
        FnCallTrace::new(
            71712880,
            "factorial".to_string(),
            "[5]".to_string(),
            1643740412580,
        ),
    );
    state.add_bind_trace(
        1709,
        BindTrace::new(
            71712880,
            "n".to_string(),
            "5".to_string(),
            vec![],
            1643740412581,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "5".to_string(), vec![3, 1, 1], 1643740412582),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "false".to_string(), vec![3, 1], 1643740412582),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "5".to_string(), vec![3, 3, 1], 1643740412583),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(
            71712880,
            "5".to_string(),
            vec![3, 3, 2, 1, 1],
            1643740412583,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "4".to_string(), vec![3, 3, 2, 1], 1643740412584),
    );
    state.add_fn_call_trace(
        1709,
        FnCallTrace::new(
            71712880,
            "factorial".to_string(),
            "[4]".to_string(),
            1643740412584,
        ),
    );
    state.add_bind_trace(
        1709,
        BindTrace::new(
            71712880,
            "n".to_string(),
            "4".to_string(),
            vec![],
            1643740412584,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "4".to_string(), vec![3, 1, 1], 1643740412584),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "false".to_string(), vec![3, 1], 1643740412585),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "4".to_string(), vec![3, 3, 1], 1643740412585),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(
            71712880,
            "4".to_string(),
            vec![3, 3, 2, 1, 1],
            1643740412585,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "3".to_string(), vec![3, 3, 2, 1], 1643740412585),
    );
    state.add_fn_call_trace(
        1709,
        FnCallTrace::new(
            71712880,
            "factorial".to_string(),
            "[3]".to_string(),
            1643740412586,
        ),
    );
    state.add_bind_trace(
        1709,
        BindTrace::new(
            71712880,
            "n".to_string(),
            "3".to_string(),
            vec![],
            1643740412586,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "3".to_string(), vec![3, 1, 1], 1643740412586),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "false".to_string(), vec![3, 1], 1643740412586),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "3".to_string(), vec![3, 3, 1], 1643740412586),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(
            71712880,
            "3".to_string(),
            vec![3, 3, 2, 1, 1],
            1643740412587,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![3, 3, 2, 1], 1643740412587),
    );
    state.add_fn_call_trace(
        1709,
        FnCallTrace::new(
            71712880,
            "factorial".to_string(),
            "[2]".to_string(),
            1643740412587,
        ),
    );
    state.add_bind_trace(
        1709,
        BindTrace::new(
            71712880,
            "n".to_string(),
            "2".to_string(),
            vec![],
            1643740412587,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![3, 1, 1], 1643740412588),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "false".to_string(), vec![3, 1], 1643740412588),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![3, 3, 1], 1643740412588),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(
            71712880,
            "2".to_string(),
            vec![3, 3, 2, 1, 1],
            1643740412588,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3, 3, 2, 1], 1643740412588),
    );
    state.add_fn_call_trace(
        1709,
        FnCallTrace::new(
            71712880,
            "factorial".to_string(),
            "[1]".to_string(),
            1643740412589,
        ),
    );
    state.add_bind_trace(
        1709,
        BindTrace::new(
            71712880,
            "n".to_string(),
            "1".to_string(),
            vec![],
            1643740412589,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3, 1, 1], 1643740412589),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "false".to_string(), vec![3, 1], 1643740412589),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3, 3, 1], 1643740412589),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(
            71712880,
            "1".to_string(),
            vec![3, 3, 2, 1, 1],
            1643740412590,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "0".to_string(), vec![3, 3, 2, 1], 1643740412590),
    );
    state.add_fn_call_trace(
        1709,
        FnCallTrace::new(
            71712880,
            "factorial".to_string(),
            "[0]".to_string(),
            1643740412590,
        ),
    );
    state.add_bind_trace(
        1709,
        BindTrace::new(
            71712880,
            "n".to_string(),
            "0".to_string(),
            vec![],
            1643740412590,
        ),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "0".to_string(), vec![3, 1, 1], 1643740412591),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "true".to_string(), vec![3, 1], 1643740412591),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3], 1643740412591),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![], 1643740412591),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3, 3, 2], 1643740412592),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3, 3], 1643740412592),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3], 1643740412592),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![], 1643740412592),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "1".to_string(), vec![3, 3, 2], 1643740412592),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![3, 3], 1643740412593),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![3], 1643740412593),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![], 1643740412593),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "2".to_string(), vec![3, 3, 2], 1643740412593),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "6".to_string(), vec![3, 3], 1643740412594),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "6".to_string(), vec![3], 1643740412594),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "6".to_string(), vec![], 1643740412594),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "6".to_string(), vec![3, 3, 2], 1643740412594),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "24".to_string(), vec![3, 3], 1643740412595),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "24".to_string(), vec![3], 1643740412595),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "24".to_string(), vec![], 1643740412595),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "24".to_string(), vec![3, 3, 2], 1643740412595),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "120".to_string(), vec![3, 3], 1643740412596),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "120".to_string(), vec![3], 1643740412596),
    );
    state.add_exec_trace(
        1709,
        ExprTrace::new(71712880, "120".to_string(), vec![], 1643740412596),
    );
}

pub fn add_cljs_compiler_1(debugger_state_arc: &Arc<Mutex<DebuggerState>>) {
    let mut state = debugger_state_arc
        .lock()
        .expect("Can't get the lock on state mutex");

    state.add_flow_form(1248,652321713,Form::new(r#"(defn -main [& args] (let [args (normalize (cli/normalize cli/default-commands args)) pred (complement #{"--repl-env" "-re"}) [pre post] ((juxt (fn* [p1__10216#] (take-while pred p1__10216#)) (fn* [p1__10217#] (drop-while pred p1__10217#))) args) [js-args args] ((juxt (fn* [p1__10218#] (take 2 p1__10218#)) (fn* [p1__10219#] (drop 2 p1__10219#))) post) repl-opt (get-js-opt js-args)] (try (apply cli/main repl-opt (concat pre args)) (finally (shutdown-agents)))))"#.to_string(), 1643897352386), 1643897352386);
    state.add_fn_call_trace(
        1248,
        FnCallTrace::new(
            652321713,
            r#"-main"#.to_string(),
            r#"[(["--compile" "hello-world.core"])]"#.to_string(),
            1643897352433,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"args"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![],
            1643897352433,
        ),
    );
    state.add_exec_trace(1248,ExprTrace::new(652321713,r#"{:main-dispatch {nil #function[cljs.cli/null-opt], "--help" #function[cljs.cli/help-opt], "-c" #function[cljs.cli/compile-opt], "--install-deps" #function[cljs.cli/install-deps-opt], "-s" #function[cljs.cli/serve-opt], "-r" #function[cljs.cli/repl-opt], "-h" #function[cljs.cli/help-opt], "-?" #function[cljs.cli/help-opt], "--main" #function[cljs.cli/main-opt], "--compile" #function[cljs.cli/compile-opt], "--serve" #function[cljs.cli/serve-opt], "--repl" #function[cljs.cli/repl-opt], "-m" #function[cljs.cli/main-opt]}, :init-dispatch {"--verbose" #function[cljs.cli/verbose-opt], "-o" #function[cljs.cli/output-to-opt], "-O" #function[cljs.cli/optimize-opt], "--repl-opts" #function[cljs.cli/repl-env-opts-opt], "-co" #function[cljs.cli/compile-opts-opt], "--eval" #function[cljs.cli/eval-opt], "-t" #function[cljs.cli/target-opt], "-w" #function[cljs.cli/watch-opt], "--init" #function[cljs.cli/init-opt], "-i" #function[cljs.cli/init-opt], "--optimizations" #function[cljs.cli/optimize-opt], "-e" #function[cljs.cli/eval-opt], "-d" #function[cljs.cli/output-dir-opt], "-ro" #function[cljs.cli/repl-env-opts-opt], "-v" #function[cljs.cli/verbose-opt], "--output-to" #function[cljs.cli/output-to-opt], "--compile-opts" #function[cljs.cli/compile-opts-opt], "--target" #function[cljs.cli/target-opt], "--watch" #function[cljs.cli/watch-opt], "--deps-cmd" #function[cljs.cli/deps-cmd-opt], "--output-dir" #function[cljs.cli/output-dir-opt]}, :groups #:cljs.cli{:main&compile {:desc "init options", :pseudos {["-re" "--repl-env"] {:arg "env", :doc "The REPL environment to use. Built-in supported values: node, browser. Defaults to browser. If given a non-single-segment namespace, will use the repl-env fn found there."}}}, :main {:desc "init options only for --main and --repl"}, :compile {:desc "init options only for --compile"}}, :main {["--install-deps"] {:fn #function[cljs.cli/install-deps-opt], :doc "Install all :npm-deps found upstream and in supplied compiler options"}, ["-r" "--repl"] {:fn #function[cljs.cli/repl-opt], :doc "Run a repl"}, ["-m" "--main"] {:fn #function[cljs.cli/main-opt], :arg "ns", :doc "Call the -main function from a namespace with args"}, ["-c" "--compile"] {:fn #function[cljs.cli/compile-opt], :arg "[ns]", :doc "Run a compile. If optional namespace specified, use as the main entry point. If --repl follows, will launch a REPL after the compile completes. If --serve follows, will start a web server that serves the current directory after the compile completes."}, ["-s" "--serve"] {:fn #function[cljs.cli/serve-opt], :arg "host:port", :doc "Start a simple web server to serve the current directory"}, [nil] {:fn #function[cljs.cli/null-opt]}, ["-h" "--help" "-?"] {:fn #function[cljs.cli/help-opt], :doc "Print this help message and exit"}}, :init {["-e" "--eval"] {:group :cljs.cli/main, :fn #function[cljs.cli/eval-opt], :arg "string", :doc "Evaluate expressions in string; print non-nil values"}, ["-w" "--watch"] {:group :cljs.cli/compile, :fn #function[cljs.cli/watch-opt], :arg "paths", :doc "Continuously build, only effective with the --compile main option. Specifies a system-dependent path-separated list of directories to watch."}, ["-co" "--compile-opts"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/compile-opts-opt], :arg "edn", :doc "Options to configure the build, can be an EDN string or system-dependent path-separated list of EDN files / classpath resources. Options will be merged left to right."}, ["-o" "--output-to"] {:group :cljs.cli/compile, :fn #function[cljs.cli/output-to-opt], :arg "file", :doc "Set the output compiled file"}, ["-d" "--output-dir"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/output-dir-opt], :arg "path", :doc "Set the output directory to use. If supplied, cljsc_opts.edn in that directory will be used to set ClojureScript compiler options"}, ["-v" "--verbose"] {:group :cljs.cli/main, :fn #function[cljs.cli/verbose-opt], :arg "bool", :doc "If true, will enable ClojureScript verbose logging"}, ["--deps-cmd"] {:group :cljs.cli/compile, :fn #function[cljs.cli/deps-cmd-opt], :arg "string", :doc "Set the node dependency manager. Only npm or yarn supported"}, ["-t" "--target"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/target-opt], :arg "name", :doc "The JavaScript target. Configures environment bootstrap and defaults to browser. Supported values: node or nodejs webworker, none"}, ["-O" "--optimizations"] {:group :cljs.cli/compile, :fn #function[cljs.cli/optimize-opt], :arg "level", :doc "Set optimization level, only effective with --compile main option. Valid values are: none, whitespace, simple, advanced"}, ["-ro" "--repl-opts"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/repl-env-opts-opt], :arg "edn", :doc "Options to configure the repl-env, can be an EDN string or system-dependent path-separated list of EDN files / classpath resources. Options will be merged left to right."}, ["-i" "--init"] {:group :cljs.cli/main, :fn #function[cljs.cli/init-opt], :arg "path", :doc "Load a file or resource"}}}"#.to_string(),vec![3, 1, 1, 1, 1], 1643897352434));
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 1, 1, 2],
            1643897352487,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 1, 1],
            1643897352489,
        ),
    );
    state.add_flow_form(1248,-730478619,Form::new(r#"(defn normalize [args] (let [[js-opt args] (normalize* args)] (concat js-opt args)))"#.to_string(), 1643897352491), 1643897352491);
    state.add_fn_call_trace(
        1248,
        FnCallTrace::new(
            -730478619,
            r#"normalize"#.to_string(),
            r#"[(["--compile" "hello-world.core"])]"#.to_string(),
            1643897352491,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            -730478619,
            r#"args"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![],
            1643897352491,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 1, 1],
            1643897352492,
        ),
    );
    state.add_flow_form(1248,7202804,Form::new(r#"(defn- normalize* [args] (if (not (cli/dispatch? cli/default-commands :main (first args))) (let [pred (complement #{"--repl-env" "-re"}) [pre post] ((juxt (fn* [p1__10306#] (take-while pred p1__10306#)) (fn* [p1__10307#] (drop-while pred p1__10307#))) args)] (if (= pre args) [nil pre] (let [[js-opt post'] (normalize* (nnext post))] (if js-opt [js-opt (concat pre post')] [[(first post) (fnext post)] (concat pre post')])))) [nil args]))"#.to_string(), 1643897352492), 1643897352492);
    state.add_fn_call_trace(
        1248,
        FnCallTrace::new(
            7202804,
            r#"normalize*"#.to_string(),
            r#"[(["--compile" "hello-world.core"])]"#.to_string(),
            1643897352493,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            7202804,
            r#"args"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![],
            1643897352493,
        ),
    );
    state.add_exec_trace(1248,ExprTrace::new(7202804,r#"{:main-dispatch {nil #function[cljs.cli/null-opt], "--help" #function[cljs.cli/help-opt], "-c" #function[cljs.cli/compile-opt], "--install-deps" #function[cljs.cli/install-deps-opt], "-s" #function[cljs.cli/serve-opt], "-r" #function[cljs.cli/repl-opt], "-h" #function[cljs.cli/help-opt], "-?" #function[cljs.cli/help-opt], "--main" #function[cljs.cli/main-opt], "--compile" #function[cljs.cli/compile-opt], "--serve" #function[cljs.cli/serve-opt], "--repl" #function[cljs.cli/repl-opt], "-m" #function[cljs.cli/main-opt]}, :init-dispatch {"--verbose" #function[cljs.cli/verbose-opt], "-o" #function[cljs.cli/output-to-opt], "-O" #function[cljs.cli/optimize-opt], "--repl-opts" #function[cljs.cli/repl-env-opts-opt], "-co" #function[cljs.cli/compile-opts-opt], "--eval" #function[cljs.cli/eval-opt], "-t" #function[cljs.cli/target-opt], "-w" #function[cljs.cli/watch-opt], "--init" #function[cljs.cli/init-opt], "-i" #function[cljs.cli/init-opt], "--optimizations" #function[cljs.cli/optimize-opt], "-e" #function[cljs.cli/eval-opt], "-d" #function[cljs.cli/output-dir-opt], "-ro" #function[cljs.cli/repl-env-opts-opt], "-v" #function[cljs.cli/verbose-opt], "--output-to" #function[cljs.cli/output-to-opt], "--compile-opts" #function[cljs.cli/compile-opts-opt], "--target" #function[cljs.cli/target-opt], "--watch" #function[cljs.cli/watch-opt], "--deps-cmd" #function[cljs.cli/deps-cmd-opt], "--output-dir" #function[cljs.cli/output-dir-opt]}, :groups #:cljs.cli{:main&compile {:desc "init options", :pseudos {["-re" "--repl-env"] {:arg "env", :doc "The REPL environment to use. Built-in supported values: node, browser. Defaults to browser. If given a non-single-segment namespace, will use the repl-env fn found there."}}}, :main {:desc "init options only for --main and --repl"}, :compile {:desc "init options only for --compile"}}, :main {["--install-deps"] {:fn #function[cljs.cli/install-deps-opt], :doc "Install all :npm-deps found upstream and in supplied compiler options"}, ["-r" "--repl"] {:fn #function[cljs.cli/repl-opt], :doc "Run a repl"}, ["-m" "--main"] {:fn #function[cljs.cli/main-opt], :arg "ns", :doc "Call the -main function from a namespace with args"}, ["-c" "--compile"] {:fn #function[cljs.cli/compile-opt], :arg "[ns]", :doc "Run a compile. If optional namespace specified, use as the main entry point. If --repl follows, will launch a REPL after the compile completes. If --serve follows, will start a web server that serves the current directory after the compile completes."}, ["-s" "--serve"] {:fn #function[cljs.cli/serve-opt], :arg "host:port", :doc "Start a simple web server to serve the current directory"}, [nil] {:fn #function[cljs.cli/null-opt]}, ["-h" "--help" "-?"] {:fn #function[cljs.cli/help-opt], :doc "Print this help message and exit"}}, :init {["-e" "--eval"] {:group :cljs.cli/main, :fn #function[cljs.cli/eval-opt], :arg "string", :doc "Evaluate expressions in string; print non-nil values"}, ["-w" "--watch"] {:group :cljs.cli/compile, :fn #function[cljs.cli/watch-opt], :arg "paths", :doc "Continuously build, only effective with the --compile main option. Specifies a system-dependent path-separated list of directories to watch."}, ["-co" "--compile-opts"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/compile-opts-opt], :arg "edn", :doc "Options to configure the build, can be an EDN string or system-dependent path-separated list of EDN files / classpath resources. Options will be merged left to right."}, ["-o" "--output-to"] {:group :cljs.cli/compile, :fn #function[cljs.cli/output-to-opt], :arg "file", :doc "Set the output compiled file"}, ["-d" "--output-dir"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/output-dir-opt], :arg "path", :doc "Set the output directory to use. If supplied, cljsc_opts.edn in that directory will be used to set ClojureScript compiler options"}, ["-v" "--verbose"] {:group :cljs.cli/main, :fn #function[cljs.cli/verbose-opt], :arg "bool", :doc "If true, will enable ClojureScript verbose logging"}, ["--deps-cmd"] {:group :cljs.cli/compile, :fn #function[cljs.cli/deps-cmd-opt], :arg "string", :doc "Set the node dependency manager. Only npm or yarn supported"}, ["-t" "--target"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/target-opt], :arg "name", :doc "The JavaScript target. Configures environment bootstrap and defaults to browser. Supported values: node or nodejs webworker, none"}, ["-O" "--optimizations"] {:group :cljs.cli/compile, :fn #function[cljs.cli/optimize-opt], :arg "level", :doc "Set optimization level, only effective with --compile main option. Valid values are: none, whitespace, simple, advanced"}, ["-ro" "--repl-opts"] {:group :cljs.cli/main&compile, :fn #function[cljs.cli/repl-env-opts-opt], :arg "edn", :doc "Options to configure the repl-env, can be an EDN string or system-dependent path-separated list of EDN files / classpath resources. Options will be merged left to right."}, ["-i" "--init"] {:group :cljs.cli/main, :fn #function[cljs.cli/init-opt], :arg "path", :doc "Load a file or resource"}}}"#.to_string(),vec![3, 1, 1, 1], 1643897352494));
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 1, 3, 1],
            1643897352500,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"["--compile" "hello-world.core"]"#.to_string(),
            vec![3, 1, 1, 3],
            1643897352501,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"false"#.to_string(),
            vec![3, 1, 1],
            1643897352501,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(7202804, r#"true"#.to_string(), vec![3, 1], 1643897352501),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 2, 1, 1],
            1643897352502,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            7202804,
            r#"pred"#.to_string(),
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 2],
            1643897352505,
        ),
    );
    state.add_exec_trace(1248,ExprTrace::new(7202804,r#"#function[cljs.main/eval10314/fn--10315/fn--10316/fn--10327/fn--10330/fn--10331/fn--10332/fn--10333]"#.to_string(),vec![3, 2, 1, 3, 0, 1], 1643897352506));
    state.add_exec_trace(1248,ExprTrace::new(7202804,r#"#function[cljs.main/eval10314/fn--10315/fn--10316/fn--10327/fn--10330/fn--10331/fn--10340/fn--10341]"#.to_string(),vec![3, 2, 1, 3, 0, 2], 1643897352508));
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"#function[clojure.core/juxt/fn--5840]"#.to_string(),
            vec![3, 2, 1, 3, 0],
            1643897352511,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 1, 3, 1],
            1643897352513,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 2, 1, 3, 0, 1, 2, 1],
            1643897352513,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 1, 3, 0, 1, 2, 2],
            1643897352514,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 1, 3, 0, 1, 2],
            1643897352514,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 2, 1, 3, 0, 2, 2, 1],
            1643897352514,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 1, 3, 0, 2, 2, 2],
            1643897352515,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"()"#.to_string(),
            vec![3, 2, 1, 3, 0, 2, 2],
            1643897352515,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"[(["--compile" "hello-world.core"]) ()]"#.to_string(),
            vec![3, 2, 1, 3],
            1643897352516,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            7202804,
            r#"pre"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2],
            1643897352516,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            7202804,
            r#"post"#.to_string(),
            r#"()"#.to_string(),
            vec![3, 2],
            1643897352516,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 2, 1, 1],
            1643897352517,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 2, 1, 2],
            1643897352517,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"true"#.to_string(),
            vec![3, 2, 2, 1],
            1643897352517,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 2, 2, 1],
            1643897352518,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"[nil (["--compile" "hello-world.core"])]"#.to_string(),
            vec![3, 2, 2],
            1643897352518,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"[nil (["--compile" "hello-world.core"])]"#.to_string(),
            vec![3, 2],
            1643897352518,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"[nil (["--compile" "hello-world.core"])]"#.to_string(),
            vec![3],
            1643897352519,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            7202804,
            r#"[nil (["--compile" "hello-world.core"])]"#.to_string(),
            vec![],
            1643897352519,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"[nil (["--compile" "hello-world.core"])]"#.to_string(),
            vec![3, 1, 1],
            1643897352519,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            -730478619,
            r#"js-opt"#.to_string(),
            r#"nil"#.to_string(),
            vec![3],
            1643897352519,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            -730478619,
            r#"args"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3],
            1643897352520,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"nil"#.to_string(),
            vec![3, 2, 1],
            1643897352520,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 2],
            1643897352521,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2],
            1643897352521,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3],
            1643897352521,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -730478619,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![],
            1643897352521,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 1],
            1643897352521,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"args"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3],
            1643897352522,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 1, 3],
            1643897352522,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"pred"#.to_string(),
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3],
            1643897352523,
        ),
    );
    state.add_exec_trace(1248,ExprTrace::new(652321713,r#"#function[cljs.main/eval10226/fn--10227/fn--10228/fn--10239/fn--10240/fn--10241/fn--10242]"#.to_string(),vec![3, 1, 5, 0, 1], 1643897352523));
    state.add_exec_trace(1248,ExprTrace::new(652321713,r#"#function[cljs.main/eval10226/fn--10227/fn--10228/fn--10239/fn--10240/fn--10249/fn--10250]"#.to_string(),vec![3, 1, 5, 0, 2], 1643897352526));
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[clojure.core/juxt/fn--5840]"#.to_string(),
            vec![3, 1, 5, 0],
            1643897352528,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 5, 1],
            1643897352528,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 1, 5, 0, 1, 2, 1],
            1643897352529,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 5, 0, 1, 2, 2],
            1643897352529,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 5, 0, 1, 2],
            1643897352529,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[clojure.core/complement/fn--5686]"#.to_string(),
            vec![3, 1, 5, 0, 2, 2, 1],
            1643897352530,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 1, 5, 0, 2, 2, 2],
            1643897352530,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 5, 0, 2, 2],
            1643897352531,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"[(["--compile" "hello-world.core"]) ()]"#.to_string(),
            vec![3, 1, 5],
            1643897352531,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"pre"#.to_string(),
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3],
            1643897352531,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"post"#.to_string(),
            r#"()"#.to_string(),
            vec![3],
            1643897352531,
        ),
    );
    state.add_exec_trace(1248,ExprTrace::new(652321713,r#"#function[cljs.main/eval10226/fn--10227/fn--10228/fn--10261/fn--10262/fn--10263/fn--10264]"#.to_string(),vec![3, 1, 7, 0, 1], 1643897352532));
    state.add_exec_trace(1248,ExprTrace::new(652321713,r#"#function[cljs.main/eval10226/fn--10227/fn--10228/fn--10261/fn--10262/fn--10269/fn--10270]"#.to_string(),vec![3, 1, 7, 0, 2], 1643897352535));
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[clojure.core/juxt/fn--5840]"#.to_string(),
            vec![3, 1, 7, 0],
            1643897352537,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 7, 1],
            1643897352537,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 7, 0, 1, 2, 2],
            1643897352537,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 7, 0, 1, 2],
            1643897352537,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 7, 0, 2, 2, 2],
            1643897352538,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 7, 0, 2, 2],
            1643897352538,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"[() ()]"#.to_string(),
            vec![3, 1, 7],
            1643897352538,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"js-args"#.to_string(),
            r#"()"#.to_string(),
            vec![3],
            1643897352538,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"args"#.to_string(),
            r#"()"#.to_string(),
            vec![3],
            1643897352538,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 1, 9, 1],
            1643897352539,
        ),
    );
    state.add_flow_form(1248,-1242896699,Form::new(r#"(defn- get-js-opt [args] (if (= 2 (count args)) (let [ns-frag (nth args 1) repl-ns (symbol (cond->> ns-frag (single-segment? ns-frag) (str "cljs.repl.")))] (try (require repl-ns) (if-let [repl-env (ns-resolve repl-ns (quote repl-env))] repl-env (throw (ex-info (str "REPL namespace " repl-ns " does not define repl-env var") {:repl-ns repl-ns}))) (catch Throwable t (throw (ex-info (str "Failed to load REPL namespace " repl-ns) {:repl-ns repl-ns} t))))) browser/repl-env))"#.to_string(), 1643897352540), 1643897352540);
    state.add_fn_call_trace(
        1248,
        FnCallTrace::new(
            -1242896699,
            r#"get-js-opt"#.to_string(),
            r#"[()]"#.to_string(),
            1643897352540,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            -1242896699,
            r#"args"#.to_string(),
            r#"()"#.to_string(),
            vec![],
            1643897352540,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -1242896699,
            r#"()"#.to_string(),
            vec![3, 1, 2, 1],
            1643897352541,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -1242896699,
            r#"0"#.to_string(),
            vec![3, 1, 2],
            1643897352541,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -1242896699,
            r#"false"#.to_string(),
            vec![3, 1],
            1643897352542,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -1242896699,
            r#"#function[cljs.repl.browser/repl-env]"#.to_string(),
            vec![3, 3],
            1643897352542,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -1242896699,
            r#"#function[cljs.repl.browser/repl-env]"#.to_string(),
            vec![3],
            1643897352544,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            -1242896699,
            r#"#function[cljs.repl.browser/repl-env]"#.to_string(),
            vec![],
            1643897352545,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[cljs.repl.browser/repl-env]"#.to_string(),
            vec![3, 1, 9],
            1643897352545,
        ),
    );
    state.add_bind_trace(
        1248,
        BindTrace::new(
            652321713,
            r#"repl-opt"#.to_string(),
            r#"#function[cljs.repl.browser/repl-env]"#.to_string(),
            vec![3],
            1643897352545,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[cljs.cli/main]"#.to_string(),
            vec![3, 2, 1, 1],
            1643897352546,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"#function[cljs.repl.browser/repl-env]"#.to_string(),
            vec![3, 2, 1, 2],
            1643897352548,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 1, 3, 1],
            1643897352549,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"()"#.to_string(),
            vec![3, 2, 1, 3, 2],
            1643897352549,
        ),
    );
    state.add_exec_trace(
        1248,
        ExprTrace::new(
            652321713,
            r#"(["--compile" "hello-world.core"])"#.to_string(),
            vec![3, 2, 1, 3],
            1643897352549,
        ),
    );
}
