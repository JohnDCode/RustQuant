<h1>📈 RustQuant 0.4</h1>
<h3>A Cross-Platform Rust CLI for Options Pricing</h3>

<p>
  <img src="https://img.shields.io/badge/Rust-🦀-orange" alt="Rust">
  <img src="https://img.shields.io/badge/version-0.4-blue" alt="Version">
  <img src="https://img.shields.io/github/license/johndcode/RustQuant" alt="License">
</p>

---

### ✨ Overview  
RustQuant is a **command-line tool written in Rust** for pricing **financial options**.  
It supports both **manual theoretical pricing** and **automatic pricing with live data**.  

🚀 Latest version (`v0.4`) includes:  
- Manual pricing for **European** & **American** options  
- Calculation of the **Greeks**  
- Default binomial steps set to `100`  

For the full development story and detailed writeup, check out the blog post here:  
[RustQuant: My Cross-Platform CLI Options Pricer](https://www.johndcode.com/posts/Options-Pricer/)  

---

### 📊 Models Implemented
- **Black–Scholes Model** → European options  
- **Cox-Ross-Rubinstein (Binomial) Model** → American & European options  

---

### ⚡ Installation
Clone the repo and build with Cargo:

```bash
git clone https://github.com/johndcode/RustQuant.git
cd RustQuant
cargo build --release
```

The binary will be located at:
`target/release/rustquant`

(Precompiled binaries for Windows and Linux [here](https://github.com/JohnDCode/RustQuant-Publish))

---

### 🛠️ Usage

RustQuant has two main modes:

#### 1. Automatic Mode (Live Pricing)

Fetches real options chain data and prices the closest matching option.

```
rustquant auto -s AAPL -k 200 --greeks
```

Select expiration date from dropdown → get fair price + Greeks


#### 2. Manual Mode (Theoretical Pricing)

Supply all parameters (spot, strike, time, rate, volatility):

```
rustquant manual -s 213.95 -k 200 -t 1 -r 0.0424 -v 0.2965 --european --greeks

```

### ⚙️ Arguments & Flags

| Command | Flag | Description |
|---------|------|-------------|
| auto | -s, --symbol <SYMBOL> | Asset ticker symbol |
|       | -k, --strike <STRIKE> | Target strike price |
|       | -n, --steps <STEPS>   | Binomial steps (default: 100) |
|       | -g, --greeks          | Display Greeks |
|       | -c, --call            | Price a call option (default) |
|       | -p, --put             | Price a put option |
| manual | -s, --spot <SPOT>    | Asset spot price |
|        | -k, --strike <STRIKE>| Option strike price |
|        | -t, --time <TIME>    | Time to expiration (years) |
|        | -r, --rate <RATE>    | Risk-free interest rate |
|        | -v, --volatility <VOL>| Implied volatility |
|        | -a, --american       | Price American option (default) |
|        | -e, --european       | Price European option |
|        | -g, --greeks         | Display Greeks |

---

### 🔮 Roadmap

Planned features for future versions:
- Automated pricing for European options with live data  
- Support for dividend-adjusted pricing  
- Expanded output formats (JSON/CSV for analysis)  

---

### 📫 Contact Me  
- 📧 Email: **johndavidabe101@gmail.com**  
- 💼 LinkedIn: [linkedin.com/in/johndcode](https://linkedin.com/in/johndcode)  
- 🧑‍💻 GitHub: [github.com/johndcode](https://github.com/johndcode)  

⭐ If you like this project, consider giving it a star on GitHub!
