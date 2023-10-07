use crate::common::{IggyCmdCommand, IggyCmdTest, IggyCmdTestCase};
use assert_cmd::assert::Assert;
use async_trait::async_trait;
use iggy::client::Client;
use predicates::str::diff;
use serial_test::serial;

struct TestQuietModCmd {}

#[async_trait]
impl IggyCmdTestCase for TestQuietModCmd {
    async fn prepare_server_state(&self, _client: &dyn Client) {}

    fn get_command(&self) -> IggyCmdCommand {
        IggyCmdCommand::new().arg("ping").opt("-q")
    }

    fn verify_command(&self, command_state: Assert) {
        command_state.success().stdout(diff(""));
    }

    async fn verify_server_state(&self, _client: &dyn Client) {}
}

#[tokio::test]
#[serial]
pub async fn test_quiet_mode() {
    let mut iggy_cmd_test = IggyCmdTest::default();

    iggy_cmd_test.setup().await;
    iggy_cmd_test.execute_test(TestQuietModCmd {}).await;
}
