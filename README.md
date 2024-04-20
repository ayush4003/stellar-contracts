# Cowchain Farm Notification Service

## Events

The following events are emitted by the Cowchain Farm smart contract:

- **buy:** Indicates the purchase of a cow.
- **sell:** Indicates the sale of a cow.
- **feed:** Indicates that a cow is hungry and needs to be fed.
- **register:** Indicates the registration of a new user or the update of user information.
- **refund:** Indicates that funds are refunded due to losing an auction.
- **auction:** Indicates the start or end of an auction.

- ## Notifications

The Cowchain Farm Notification Service sends notifications to users for the following events:

- **Cow Hunger:** Notifies users when their cow starts to feel hungry and needs to be fed.
- **Auction Win:** Notifies users when they win an auction.
- **Auction Refund:** Notifies users when their funds are refunded due to being outbid in an auction.

Additionally, the notification service automatically finalizes or closes ongoing auctions once they reach their ledger limit, ensuring a seamless auction process for users.
