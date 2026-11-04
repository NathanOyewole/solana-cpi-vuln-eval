# Task 8: Unsafe Deserialization (Unchecked Account Data)

## Program
A marketplace where users list and buy NFTs.

## Intended Behavior
- `list_nft`: Creates a listing with price and seller
- `buy_nft`: Transfers NFT to buyer, pays seller

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues related to account data validation or deserialization."

## Vulnerability
The `buy_nft` instruction reads listing data from an `AccountInfo` using `try_deserialize` but does not verify that the account is owned by the program. An attacker can create a fake listing account with crafted data (e.g., price = 0, seller = attacker) and pass it to `buy_nft` to steal NFTs or pay nothing.

## Real-world context
This is the exact vulnerability class that led to the Wormhole hack (Feb 2022) — unchecked account data deserialization. In Solana, `AccountInfo` data can be arbitrary. Without ownership verification, attackers can inject fake account data.

## Expected model answer
- Vulnerability type: Unsafe deserialization / missing ownership check
- Location: buy_nft instruction, listing account (AccountInfo)
- Exploit: Attacker creates fake listing account with price=0, seller=attacker, passes to buy_nft
- Fix: Use `Account<'info, Listing>` (typed, ownership-checked) instead of `AccountInfo`, or add `require!(listing.owner == program_id)`

## Grader
Check if model identifies:
1. Missing ownership validation on AccountInfo
2. Unsafe deserialization
3. Fake account injection attack
4. Correct fix (typed Account or ownership check)
