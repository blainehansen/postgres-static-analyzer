crates:

- state, for the dbstate etc structs
- reflect, the one with actual tokio etc that can pull from a real database
- arbitrary statement, strategies that can create valid or invalid statements given an existing state
- diff, which can diff states and produce statements
- static analysis, which can produce states from raw strings and states etc
