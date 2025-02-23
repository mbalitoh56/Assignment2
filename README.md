# Assignment2
# Simple Linear Regression in Rust

## What This Project Does
This project creates a simple Linear Regression model in Rust using the `burn` library. The model learns from fake data that follows the equation `y = 2x + 1` but with some added randomness (noise). After training, I test the model and plot the results using `textplots`.

## How to Set Up
### What You Need
- Install Rust and Cargo: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- Add these to your `Cargo.toml` file:
  ```toml
  [dependencies]
 burn = { version = ”0.16.0”, features = [”wgpu”,”train”] }
 burn-ndarray = ”0.16.0”
 rand = ”0.9.0”
 rgb = ”0.8.50”
 textplots = ”0.8.6”
  ```
### How to Run It
1. Download or copy the code into a Rust project:
   ```sh
   git clone <repository-url>
   cd <project-folder>
   ```
2. Run the project:
   ```sh
   cargo run
   ```

## How It Works
1. **Making Fake Data**
   - I create `x` and `y` values where `y = 2x + 1`.
   - I add some small random changes to `y` so it looks more like real-world data.
   
2. **Building the Model**
   - I used `burn` to make a simple model that can learn the equation.
   - The model makes guesses and learns from mistakes using Mean Squared Error (MSE).
   
3. **Training the Model**
   - The Adam optimizer helps the model get better over time.
   - I check the loss to make sure learning is happening.
   
4. **Testing and Showing Results**
   - The model predicts values for new `x` inputs.
   - I used `textplots` to draw a simple chart of the results.

## Problems I Faced
- **Setting Up `burn`**: Learning how to use the library was tricky at first.
- **Training Issues**: I had to change some settings to make the model learn properly.
- **Making a Chart**: Formatting the data correctly for `textplots` took some effort.

## Where I Got Help
- [Burn Library Documentation](https://burn.dev/docs)
- Rust forums and online guides
- AI tools helped fix errors and organize the code mostly chatGPT was used for code
- Unfortunately I could not make the program run I was running to some errors I did not know how to fix.

## What I Learned
### Help I Used
- AI tools helped me structure and debug the code.
- Online guides explained how `burn` works.

### What We Discovered
- Rust has strict rules, so understanding data types is important.
- Small changes in training settings can make a big difference.
- Graphs make it easier to see if the model is learning properly.
- AI tools cannot be trusted, they write wrong code and they can't fix your problems all the time
- AI tools provide answers even if the answers are wrong.
- This assingment overall was very challanging for me especially with no experience coding with Rust.


