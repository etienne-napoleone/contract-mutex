# contract-mutex

A cosmwasm contract implementing a really simple mutex lock interface.

Is it a good idea? Probably not. Should you use it in prod? No.

## Messages

### Instantiate

Instantiate a lock with a given whitelist.

```json
{
    "whitelist": ["terra12345", "terra16789"]
}
```

### Execute

#### Lock

Try to get the lock.

Will fail if:

- Sender is not in the whitelist
- Is already locked

```json
{
    "lock": {}
}
```

#### Lock

Try to release de lock.

Will fail if:

- Not locked
- Not owner of the lock

```json
{
    "unlock": {}
}
```

### Query

#### Lock

Query the current lock state.

```json
{
    "lock": {}
}
```

response:

```json
{
    "since_height": 123,
    "owner": "terra12345"  // or none if not locked
}
```

#### Whitelist

Query the lock whitelisted addresses.

```json
{
    "whitelist": {}
}
```

response:

```json
{
    "members": ["terra12345", "terra16789"]
}
```
