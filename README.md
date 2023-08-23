# 🔧 Work in Progress 🔨

# 🎲🔐Encryption by Random

*The purpose of this project is to consolidate the learned Rust basics and to make stronger version of the previous [template-project](https://github.com/maestroviktorin/encbyran-py).*

## What is the *Encryption Method*?

The Encryption Method is based on the Caesar Cipher.  
The program reads the words from an *Original File* and represents each byte in the following format:

*Encrypted File*

```
<randomly generated action word><randomly shifted byte value>
```

*Decryptor*

```
<magnitude of byte value shift>
```

Let's break down what is what.

**`<randomly generated action word>`**

The program randomly generates two collections of `Action Word`s - `plus` and `minus` (their size can be configured by `--approximate-action-set-size` parameter). When encrypting a byte, either to add or to subtraction the byte value is decided and based on this decision a random `Action Word` is chosen from the corresponding collection. Both collections are written to `Decryptor` after generation.
Size of  so-called `plus` and `minus` can be configured by specifying `--approximate-action-set-size` parameter of the `encrypt` command.

**`<randomly shifted byte value>`**

When an action (addition or subtraction) is chosen, the byte value is changed by randomly generated number.
The range which magnitude is within can be configured by specifying `--shift-range` parameter of the `encrypt` command.

**`<magnitude of byte value shift>`**

In order for an *Encrypted File* to be decrypted in the future magnitude of each shifted byte value is written into *Decryptor*. Each `0` in *Decryptor* corresponds to `'\n'` in the *Original File*.

Now let's simulate a real usage.

*Original File*

```
Hi
there.
```

*Encrypted File*

```
IEzQ-60 lJmh151 // "Hi"
IEzQ159 foIo205  // It's not a real word, but just an encrypted notation of new line.
foIo265 SnVe4 foIo106 lJmh198 IEzQ-40 foIo226 // "there."
SnVe-110 lJmh76  // And it's too.
```

*Decryptor*

```
["Cdyk", "foIo", "lJmh"]  // All the `plus` `Action Words`.
["SnVe", "eeWW", "IEzQ"]  // All the `minus` `Action Words`.
132 46  // "Hi"
0  // A `0` by which the program can distinguish a real word from a new line.
149 100 5 84 141 180 // "there."
0  // And here as well.
```

`Hi` from the *Original File* became `IEzQ-60 lJmh151` in the *Encrypted File*. Let's look at `IEzQ-60` more thoroughly. Remember the `<randomly generated action word><randomly shifted byte value>`.

`IEzQ` means that the byte value has been subtracted as we can see it in the `minus` `Action Word`s collection in the *Decryptor*.
`-60` is the shifted byte value. Relying on the `IEzQ` we can claim that it's exactly subtracted byte value.
In order to get the `H` back, we perform `-60 + 132 = 72`, which is indeed the byte value of `H`. `132` is obtained from the corresponding position in the *Decryptor*, it's a `<magnitude of byte value shift>`.

## Usage

❗ *Warning*: This tool is not yet distributed, so it's not possible to use it as `encbyran` in the terminal, but it is possible when replacing `encbyran` with `cargo run --` ang pre-installed `cargo`.

***There are two commands - `encrypt` and `decrypt`.  Both of them are provided with `--help`.***

**Some basic examples of usage**

Encrypt the `sample.txt` file with all the encryption parameters set by default.

```
encbyran encrypt --file sample.txt
```

Encrypt the `passphrase.txt` file, convert all the letters to the lowercase, each byte value can be shifted by at least than 34 and no more than 56, there can be 78 different `Action Word`s for both addition and subtraction.

```
encbyran encrypt -f passphrase.txt --to-lower --shift-range 34,56 --approximate-action-set-size 78
```

Decrypt the `encrypted-password.txt` file using the `decryptor-for-password.txt` as *Decryptor*. Both produced by an `encbyran encrypt ...` before.

```
encbyran decrypt --file encrypted-password.txt --decryptor decryptor-for-password.txt
```

## Credits

Thanks to [The Rust Programming Language Discord Server Community](https://discord.gg/rust-lang) and [Stack Overflow Community](https://stackoverflow.com) for the help they provided.

## License

[GNU General Public License v3.0 only (GPL-3.0-only)](LICENSE.md)

---
If you have any ideas on how to make this tool better be sure to write [me](https://www.github.com/maestroviktorin).
