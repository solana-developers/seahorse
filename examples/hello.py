# hello
# Built with Seahorse v0.2.0
#
# Greets users to Solana by printing a message and minting them a token!

from seahorse.prelude import *

declare_id('')


class Hello(Account):
  bump: u8


@instruction
def init_token(owner: Signer, hello: Empty[Hello], mint: Empty[TokenMint]):
  
  bump = hello.bump()

  hello = hello.init(
    payer=owner,
    seeds=['hello']
  )

  mint.init(
    payer = owner,
    seeds = ['hello-mint'],
    decimals = 0,
    authority = hello
  )

  hello.bump = bump


@instruction
def init_user_token_account(user: Signer, user_acc: Empty[TokenAccount], mint: TokenMint):
    
  user_acc.init(
    payer = user,
    seeds = ["hello-token", mint],
    mint = mint,
    authority = user
  )


@instruction
def say_hello(user_acc: TokenAccount, hello: Hello, mint: TokenMint):

  bump = hello.bump
  
  mint.mint(
    authority = hello,
    to = user_acc,
    amount = u64(1),
    signer = ['hello', bump]
  )

  print(f'Hello {user_acc.authority()}, have a token!')