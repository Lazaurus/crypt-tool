/*

Decepticond S 
GSM Tools for nonspecific usecases

*/

use clap::Parser;
mod xortool;
mod aes;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cmd {
    #[command(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(about = "XOR sub-command. Takes 2 binary values, XORs them, and outputs the result.")]
    Xortool,

    #[command(about = "Outputs a list of commands ")]
    Cmdlist,

   // #[command(about = "AES 128, 192, 256 encoder with given key.")] 
    //Aesencoder, 

    //#[command(about = "unknown ")]
    //Untool2,
}

fn main() {
    let args = Cmd::parse();

    if args.subcmd.is_none() {
        println!("\n{WELCOME}\n");
    }

    match args.subcmd {
        Some(subcmd) => match subcmd {
            SubCommand::Xortool => xortool::xor_tool(),
            SubCommand::Cmdlist => println!("{}", CMDLIST),
        },
        None => {
            println!("No targets given."); 
        }
    }
}

const WELCOME: &str = r" Welcome to the...
              .;.       ,;        '.       .;.              
              .ld,.     'xl.    .:c.     .'lc.            
               :00Okdc,..oOo;,,;cd:...,;cldd;               
               ,OX0xdxkk::xxl:;cdo;;llc::odo'               
               .kNKxcccl,'lx;..;xl',;;;,cdxl.               
                oXKkollc;.:xl::ld:.,:::coxx:                
                .o0kddxoc;;:oddo:;;coooldxc.                
                .;cdl;'';cc:;;;;:cc;'';loc;.                
                .ll,coc.   ,llll,.  'lxo:oo.                
                 :xl,;odc'.:dddd:.'lxxc:x0c                 
                 'odo;,cddodddddddxko:lk0k,                 
                 .ldddc,:odddddxxkxc:dOOOo.                 
                  ,ldddo;,lddxxxkd:ck0Okd;                  
                    .,:loc,cxkkkl:dkdc;..                   
                        .',.;oo:';;.                     
                
                  Cryptography cmdline tool. 
Run crypt-tool cmdlist for a list of subcommands and their descriptors.";


const CMDLIST: &str = "List of subcommands.

        [1] xortool
             - usage: crypt-tool xortool <input1> <input2>
        Description: Takes two binary inputs, XOR's them, and then prints the output.

        [2] cmdlist
            -usage: crypt-tool cmdlist
        Description: Outputs a list of subcommands and their descriptors.";
