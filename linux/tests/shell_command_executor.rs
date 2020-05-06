use linux::{IsShellCommandExecutor, ShellCommandExecutor};
use speculate::speculate;

speculate! {
  describe "ShellCommandExecutor" {
    describe "#execute" {
      before {
        let target_file = {
          let mut tmp = std::env::temp_dir();
          tmp.push("hoge.txt");
          tmp
        };
        std::fs::remove_file(target_file.clone()); // テストがコケてafterが実行されなかった場合にここでファイルを消しておかないと一生テストが通らなくなる
      }

      after {
        std::fs::remove_file(target_file);
      }

      it "executes command" {
        ShellCommandExecutor.execute(format!("echo aaaa >> {}", target_file.to_str().unwrap()));
        std::thread::sleep(std::time::Duration::from_millis(10)); // コマンドが終了するまでの時間待つ
        let file_content = std::fs::read_to_string(target_file.clone()).unwrap();
        assert_eq!(file_content, String::from("aaaa\n"));
      }
    }
  }
}
