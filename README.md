
# Key to animal

Derive a silly animal name from a 32 byte hash or key

```
let result = key_to_animal::key_to_name(&[
    132, 122, 1, 1, 1, 1, 1, 1, 32, 2, 2, 2, 3, 4, 5, 6, 7, 8, 2, 3, 4, 5, 3, 4, 4, 5, 6,
    4, 5, 6, 3, 2,
]);
assert_eq!(result, "greyishLemur".to_string());
```

The idea is to have a recognisable/memorable name made up of an adjective and the name of an animal.
