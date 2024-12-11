# README

## はじめに

このレッスンでは、Aleph Zeroスマートコントラクトにおけるアクセス制御の検証の重要性について説明します。

## 前提条件

このレッスンを理解し、完了するには、レッスン1: *はじめに* を修了していることが推奨されます。追加の準備として、公式の [ink! ドキュメント](https://use.ink/) を参照することができます。

## 目標と成果

このレッスンの終わりまでに、以下のことが学べます：

- Web3におけるアクセス制御の概念
- アクセス制御が不十分な場合の結果
- 弱いアクセス制御を悪用する方法
- アクセス制御への攻撃を軽減する方法

## 演習: 脆弱なスマートコントラクト

スマートコントラクトにおけるアクセス制御は、特定の関数や状態にアクセスできるのが認可されたユーザーのみに限られることを保証します。適切なアクセス制御がない場合、外部ユーザーがコントラクトのグローバル状態を変更でき、ロジックが破壊される可能性があります。

### サンプルコントラクト

Bobは、Aleph Zero上で展開するためのスマートコントラクトを作成しました。以下は、`set_price`と`set_owner`関数が公開されており、適切なアクセス制御がない例です：

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod price {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Price {
        total_supply: u32,
        price: u32,
        owner: AccountId,
        balances: Mapping<AccountId, u32>,
    }

    impl Price {
        #[ink(constructor)]
        pub fn new(supply: u32, price: u32) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &supply);

            Self {
                total_supply: supply,
                price,
                owner: caller,
                balances,
            }
        }

        #[ink(message)]
        pub fn set_price(&mut self, price: u32) {
            self.price = price;
        }

        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: AccountId) {
            self.owner = new_owner;
        }
    }
}
```

### セキュリティ上の脆弱性

このコントラクトは、`set_price`や`set_owner`を呼び出せるのが所有者だけであることを確認するための検証を欠いています。そのため、任意のユーザーがコントラクトの価格や所有権を変更することが可能です。

### アクセス制御が不十分な場合の結果

不適切なアクセス制御は、次のような結果を招く可能性があります：

- コントラクトの重要な状態が変更される
- 認可されていないユーザーによる所有権の取得
- 金銭的または機能的利益を目的としたコントラクトの悪用

## シミュレートされた攻撃

コンパイル済みのコントラクトをダウンロードし、Aleph Zeroテストネットにデプロイします。次の方法で脆弱性を悪用してみましょう：

1. 別のアカウントを使用して価格を変更する
2. 所有権を別のアカウントに割り当てる

### 攻撃のセットアップ

1. `cargo-contract`をインストールします：
   ```bash
   cargo install cargo-contract --version 2.0.0-beta.1
   ```
2. コントラクトを作成してコンパイルします：
   ```bash
   cargo contract new price
   ```
3. デフォルトの`lib.rs`コードを脆弱なコントラクトコードに置き換えます。

4. 次のコマンドを使用してコントラクトをビルドしてデプロイします：
   ```bash
   cargo +nightly contract build --release
   cargo contract instantiate --constructor new --args 1000 450
   ```

### 悪用

別のアカウントを使用して以下を実行します：

1. `set_price`を呼び出して価格を変更する
2. `set_owner`を呼び出して所有権を再割り当てする

## セキュアな解決策

これらの脆弱性に対処するには、アクセス制御を適切に実装し、所有者のみが重要なコントラクト状態を変更できるようにします。更新された実装では、呼び出し元がコントラクトの所有者であることを確認するためのチェックを含めます：

```rust
#[ink(message)]
pub fn set_price(&mut self, price: u32) {
    if self.owner == self.env().caller() {
        self.price = price;
    }
}

#[ink(message)]
pub fn set_owner(&mut self, new_owner: AccountId) {
    if self.owner == self.env().caller() {
        self.owner = new_owner;
    }
}
```

### セキュアなコントラクト実装

以下は、セキュアなバージョンのコントラクトです：

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod price {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Price {
        total_supply: u32,
        price: u32,
        owner: AccountId,
        balances: Mapping<AccountId, u32>,
    }

    impl Price {
        #[ink(constructor)]
        pub fn new(supply: u32, price: u32) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &supply);

            Self {
                total_supply: supply,
                price,
                owner: caller,
                balances,
            }
        }

        #[ink(message)]
        pub fn set_price(&mut self, price: u32) {
            if self.owner == self.env().caller() {
                self.price = price;
            }
        }

        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: AccountId) {
            if self.owner == self.env().caller() {
                self.owner = new_owner;
            }
        }
    }
}
```

このセキュアなコントラクトを再デプロイし、認可されていないユーザーが価格や所有権を変更できなくなったことを確認してください。