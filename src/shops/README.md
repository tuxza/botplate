# What's in this directory?

list of shops/
- README.md - the file you're reading
- manage.rs - contains the parent 'shop' and 'items' command
  - includes the /shop create, and delete commands
- helpers.rs - helper functions for channels
  - includes the create_shop, delete_shop, and check_category functions
- db.rs - database related functions
- mod.rs - mod file. 
  - Exports 'buy', 'sell', 'db', 'list', and 'manage' publically.
- list.rs - /items list command, may be moved later.

list of shops/sell/
- db.rs - includes add_item
- items.rs - has the /items sell, and remove commands.
- mod.rs - mod file. Exports 'items' publically.

list of shops/buy/
- db.rs - includes db_get_item, db_get_item_by_id, and db_remove_item
- items.rs - has the /buy command
- mod.rs - mod file. Exports 'items' publically. (crazy huh.)
