# Rust Biofile CLI Maker Template
A simple repository for building Biology CLIs in Rust fast. The motivation for this repository are 1) Sorting through the many crates to read and write bioinformatics files and 2) rust htslib is a wrapper rather than a rust implementation. Here we have rust templates to start making CLIs, with examples from each crate.

Follows the structure of the [Rust CLI Tutorial](https://rust-cli.github.io/book/index.html).

Features:
<ul>
<li>Main.rs and Cargo.toml include commented code to create CLIs based on file types, just uncomment for the file type you're working with.</li>
<li>Rust github CI workflow to create releases for Windows, Mac, and Ubuntu automatically.</li>
<li>High performance libraries for reading and writing each file type. </li>
</ul>


# Get Started: Steps to a CLI and a CI repo

Clone the repository

Uncomment main.rs for file type

Uncomment Cargo.toml for file type

Run `Cargo build`

Run `Cargo run`

Find your CLI in Rust_Bio_CLI_Template/target/debug. The default name for the program is Rust_Bio_CLI_Template.

For CI:

replace all occurrences of 'Rust_Bio_CLI_Template' with what you would like to name your releases

# Crates Used by Filetype

FASTA/FASTQ: Seq_io 
BAM/SAM: Bam
Bigwig/wig: bigtools
Bigbed/bed: bigtools
GFF: Bio
