# Practice 1

## question

这次的作业可以直接在 [Solana Playground](https://beta.solpg.io/) 上完成。

1. 在 Solana Playground 上的命令行中领取空投，网络默认选择 DevNet，命令行的使用方法可以参考 [Solana CLI Reference and Usage](https://docs.solanalabs.com/cli/usage)；

2. 在命令行中实现转账（transfer）指令，复制这两次使用产生的签名，在 solana 区块链浏览器中获取交易的信息，并分析其中显示的字段，理解其中的意思，不清楚的可以查看 [Terminology](https://solana.com/docs/terminology)。

3. 根据这个[教程](https://beta.solpg.io/tutorials/hello-solana)，利用合约写 MEMO，可以参考 [hello world program](../zh-chs/hello-world-program.md)。

拓展：

playground 上还有[其他教程](https://beta.solpg.io/tutorials/hello-solana)，可以尝试去学习和完成。

有什么问题可以在微信群上联系老师，或者在 [tg](https://t.me/+0vMnc22l8Ns0Y2Q1) 上提问。



## answer

1. 连接钱包 

   ![image-20240302182544631](.\assets\image-20240302182544631.png)

2. 空投

   ```sh
   solana airdrop 1
   ```

3. 转账

   ```sh
   solana transfer --allow-unfunded-recipient Exroadj1iHFjwLKTEkqjSto4XNLJZsPPnAcvoGxVmvh6 1
   ```

   交易：`2kfej2vqYw5msyzjVtbHxepd1wPhn4nKdCDxRsi8MZ4J1ZkqiWvWA9VEdXuR6njP6wzRXbjr69LQeV45TPstQkoC`

   ![image-20240302182229079](.\assets\image-20240302182229079.png)

4. 写 memo

   - 部署的代码

     ```rust
     use solana_program::{
         account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
     };
     
     entrypoint!(process_instruction);
     
     pub fn process_instruction(
         program_id: &Pubkey,
         accounts: &[AccountInfo],
         instruction_data: &[u8],
     ) -> ProgramResult {
         msg!("Hello, world!");
     
         Ok(())
     }
     
     ```

   - 部署地址： `9YUGgkFi9YqLM6hqGjqCs1j5L9kk4vQd8dQh2iHgQh?cluster=devnet`

   - 完整练习

     