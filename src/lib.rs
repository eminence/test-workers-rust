use worker::*;
mod utils;

mod counter {
    use worker::*;

    #[durable_object]
    pub struct Counter {
        counter: u64,
        state: State,
        init: bool,
        _env: Env, // access `Env` across requests, use inside `fetch`
    
    }
    
    #[durable_object]
    impl DurableObject for Counter {
        fn new(state: State, env: Env) -> Self {
            console_log!("Creating Counter DU");
            Self {
                counter: 0,
                state,
                init: false,
                _env: env,
            }
        }
    
        async fn fetch(&mut self, _req: Request) -> Result<Response> {
            console_log!("DU.fetch");
            if !self.init {
                self.init = true;
                self.counter = self.state.storage().get("count").await.unwrap_or(0);
            }
            // log_request(&req);
            self.counter += 1;
            self.state.storage().put("count", self.counter).await?;
            Response::ok(format!("counter={}", self.counter))
        }
    }
    
}

#[event(fetch)]
pub async fn main(_req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();


    let namespace = env.durable_object("COUNTER")?;
    let stub = namespace.id_from_name("A")?.get_stub()?;
    let _resp = stub.fetch_with_str("/test").await?;
    Response::ok(format!("counter={:?}", "test"))
    
}