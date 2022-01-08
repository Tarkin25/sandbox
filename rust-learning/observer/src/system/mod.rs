mod user;
mod trade_executor;

#[cfg(test)]
mod tests {
    use crate::system::trade_executor::TradeExecutor;
    use crate::system::user::{CreateUser, UserManagement};

    #[test]
    fn it_works() {
        let mut user_management = UserManagement::default();
        let mut trade_executor = TradeExecutor::default();

        {
            user_management.user_created.subscribe(&mut trade_executor);
            user_management.create_user(CreateUser { username: "john.doe".into(), api_key: "abc-def".into() });
        }

        println!("user_keys: {:?}", &trade_executor.user_keys);
    }

}