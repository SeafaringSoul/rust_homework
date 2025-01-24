import * as anchor from "@coral-xyz/anchor"; // Anchor 的核心库，用于与 Solana 区块链交互
import { Program } from "@coral-xyz/anchor"; // Anchor 提供的程序接口类型
import { SolHomework } from "../target/types/sol_homework"; // 导入 IDL 文件，类型化智能合约
import assert from "assert"; // Node.js 原生断言库，用于检查测试结果

// 测试用例的描述（Mocha 测试框架的语法）
describe("sol_homework", () => {
  // 1. 初始化 Anchor 提供器
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider); // 设置全局提供器，后续所有操作都使用这个环境

  // 2. 绑定我们编写的智能合约到测试上下文中
  const program = anchor.workspace.SolHomework as Program<SolHomework>;

  // 3. 生成一个新的 Solana 账户，用来存储 StringAccount 数据
  const stringAccount = anchor.web3.Keypair.generate();

  // 测试用例 1：测试创建操作
  it("Creates a new StringAccount", async () => {
    const initString = "Hello, Anchor!"; // 定义初始化字符串

    // 调用智能合约的 `create` 方法
    await program.methods
      .create(initString) // 调用程序的 `create` 方法，传递初始化字符串
      .accounts({
        stringAccount: stringAccount.publicKey, // 指定需要操作的账户（存储字符串的账户）
        user: provider.wallet.publicKey, // 付款账户（用户签名账户）
        systemProgram: anchor.web3.SystemProgram.programId, // 系统程序（Solana 的系统账户，必须传入）
      })
      .signers([stringAccount]) // 指定需要签名的账户（新建的账户需要签名以证明合法性）
      .rpc(); // 发送交易到区块链

    // 从链上获取刚刚创建的账户数据
    const account = await program.account.stringAccount.fetch(
      stringAccount.publicKey
    );

    // 使用 assert 确保数据与预期一致
    assert.strictEqual(
      account.data,
      initString,
      "StringAccount data does not match the initial string."
    );
  });

  // 测试用例 2：测试更新操作
  it("Updates the StringAccount", async () => {
    const newString = "Updated String"; // 定义更新后的字符串

    // 调用智能合约的 `update` 方法
    await program.methods
      .update(newString) // 传递新字符串
      .accounts({
        stringAccount: stringAccount.publicKey, // 指定需要更新的账户
        user: provider.wallet.publicKey, // 用户签名账户
      })
      .rpc(); // 发送交易

    // 再次从链上获取账户数据
    const account = await program.account.stringAccount.fetch(
      stringAccount.publicKey
    );

    // 确保数据被正确更新
    assert.strictEqual(
      account.data,
      newString,
      "StringAccount data was not updated correctly."
    );
  });

  // 测试用例 3：测试读取操作
  it("Reads the StringAccount data", async () => {
    // 直接从链上读取账户数据
    const account = await program.account.stringAccount.fetch(
      stringAccount.publicKey
    );

    // 检查读取到的数据是否正确
    assert.strictEqual(
      account.data,
      "Updated String",
      "Read operation failed to fetch the correct data."
    );
  });

  // 测试用例 4：测试删除操作
  it("Deletes the StringAccount data", async () => {
    // 调用智能合约的 `delete` 方法
    await program.methods
      .delete()
      .accounts({
        stringAccount: stringAccount.publicKey, // 指定需要操作的账户
        user: provider.wallet.publicKey, // 用户签名账户
      })
      .rpc(); // 发送交易

    // 从链上获取账户数据
    const account = await program.account.stringAccount.fetch(
      stringAccount.publicKey
    );

    // 检查账户数据是否被清空
    assert.strictEqual(
      account.data,
      "",
      "StringAccount data was not deleted correctly."
    );
  });
});
