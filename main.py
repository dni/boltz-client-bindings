import json
import boltz_client_bindings

client = boltz_client_bindings.Client("http://127.0.0.1:9001/v2")

keys = boltz_client_bindings.new_keys()
swap = client.create_submarine_swap(
    "BTC",
    "BTC",
    "lnbcrt993970n1pn89k9lpp5hprvmnu5745yva65dxxfgmuy8k0mk0kkgevk9j5xvzy29q2vncssdq5g9kxy7fqd9h8vmmfvdjscqzzsxqyz5vqsp5cef2qp99z456tlg9kcpuraea8kc4r7x6022whh8c8t62dfslzdus9qxpqysgq0xjuscnmeu7t55svw6xht7w236qwez0lw4fdnlk6ej64mxw2rk8kg7as9u5sqahvf2vanu58wju7zpnqs63hnjxkt2eq65mpvtnpjdqpm2d7xy",
    keys[1]
)
print("new swap", swap.id)

script = boltz_client_bindings.BtcSwapScript.from_submarine_response(
    swap,
    keys[1]
)
print("is submarine", script.is_submarine())

"""
swap_tree = boltz_client_bindings.SwapTree(
    claim_leaf=boltz_client_bindings.Leaf(version=192, output="a91498e4e263ec4a02ed1fb12a84d31189e146156e5b88202cce59e51d4df2c0e0ae76173e30cc8b385e3fc569465738dca5c9784c9265bcac"),
    refund_leaf=boltz_client_bindings.Leaf(version=192, output="20b18e2fae2f8d990c47dfd5d938fbc7cfed14a765b6733274f8aa6518f3267ab1ad027904b1"),
)

res = boltz_client_bindings.CreateSubmarineResponse(
    False,
    "bcrt1pqukjf36n4yjj76l8gt7kcrl585qd6zsls3mfphs34evlse83h7jqe7yrvt",
    "bitcoin:bcrt1pqukjf36n4yjj76l8gt7kcrl585qd6zsls3mfphs34evlse83h7jqe7yrvt?amount=0.00102918&label=Send%20to%20BTC%20lightning",
    bytearray.fromhex("022cce59e51d4df2c0e0ae76173e30cc8b385e3fc569465738dca5c9784c9265bc"),
    102918,
    "bJZzkSfyUEGY",
    swap_tree=swap_tree,
)



"""