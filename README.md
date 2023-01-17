# shhh

A shell wrapper that keeps secrets out of build and CI logs.

## Usage

Change the shebang line of your script:

```bash
#!/usr/bin/env shhh
echo "lol check out my private key:"
cat ~/.ssh/id_ed25519
```

Running it should mask secrets in the output:

```
$ ./your-script.sh
lol check out my private key:
*** start(private-ssh-key) ***
*** private-ssh-key ***
*** private-ssh-key ***
*** private-ssh-key ***
*** private-ssh-key ***
*** private-ssh-key ***
*** end(private-ssh-key) ***
7 lines redacted
```
