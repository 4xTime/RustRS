use std::io;
use std::io::{Read,Write};
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

fn AddGeneric<T: std::ops::Add<Output=T>>(IFB:T,ISC:T)->T{return IFB+ISC;}
fn SubGeneric<T: std::ops::Sub<Output=T>>(IFB:T,ISC:T)->T{return IFB-ISC;}
fn MultGeneric<T: std::ops::Mul<Output=T>>(IFB:T,ISC:T)->T{return IFB*ISC;}
fn DivGeneric<T:std::ops::Div<Output=T>>(IFB:T,ISC:T)->T{return IFB/ISC;}
fn ModGeneric<T: std::ops::Rem<Output=T>>(IFB:T,ISC:T)->T{return IFB%ISC;}
fn ExpGeneric<T: std::ops::BitXor<Output=T>>(IFB:T,ISC:T)->T{return IFB^ISC;}

// Try to get number from source
fn GetNumbers(arg:String, Start:std::time::Instant){
    // Token store Token in a row
    let mut Token:Vec<char> = Vec::new();
    //Get chars from string
    let Buff:Vec<char> = arg.chars().collect::<Vec<_>>();

    let mut Place:Vec<u32> = Vec::new();
    // Storage for first number line
    let mut FirstBuff:Vec<String> = Vec::new();
    // Storage for second number line
    let mut SecBuff:Vec<String> = Vec::new();

    let mut PlusSign = false;
    let mut AfterSignLeave = true;
    for (i,x) in arg.chars().enumerate(){
        // If AfterSignLeave = false then Token get push into vector
        if(x == '+' || x == '-' || x == '*' || x =='/'
        ||x =='%' || x =='^'){AfterSignLeave = false;Token.push(x);}
        //  If is true FirstBuff gets first numbers (numbers before Token)
        if(AfterSignLeave == true){FirstBuff.push(x.to_string());}
        if(x == ';'){AfterSignLeave = true;}


        //for second numbers
        if(x == '+' || x == '-' || x == '*' || x =='/'
        ||x =='%' || x =='^'){PlusSign = true;}
        //Extract numbers after first numbers
        if(PlusSign == true){
            if(x == ';'){PlusSign = false;}
            if(x !='+' && x !='-'&&x !='*' && x !='/'
            &&x !='%' && x !='^'){SecBuff.push(x.to_string());}
        }
    }
    SelectNumbersFromVec(FirstBuff,SecBuff,Token,arg,Start);
}

fn ConvertToInt(StringFirstBuff:String,StringSecBuff:String,Token:char){
    //Storage for sum
    let mut IntFirstBuff:i32 =0;

    //Create vector to containe string as a bytes for conversion into str
    let VecBuff:Vec<u8> = StringSecBuff.as_bytes().to_vec();
    //Create str type for StringFirstBuff and StringSecBuff
    let StrSecBuff = std::str::from_utf8(&VecBuff).unwrap();
   
    let VecBuff:Vec<u8> = StringFirstBuff.as_bytes().to_vec();

    let StrFirstBuff = std::str::from_utf8(&VecBuff).unwrap();

    //Delete \r to get only numbers and convert into intiger type
    let StrFirstBuff = StrFirstBuff.replace("\r","");    
    if(Token == '/' || Token =='%' && StrFirstBuff != "[" || StrSecBuff != "]"){
        let FloatFirstBuff:f32 = StrFirstBuff.to_string().parse().unwrap();
        let FloatSecBuff:f32 = StrSecBuff.to_string().parse().unwrap();
        if(Token == '/'){println!("{}",DivGeneric(FloatFirstBuff,FloatSecBuff))};
        if(Token == '%'){println!("{}",ModGeneric(FloatFirstBuff,FloatSecBuff))};
    }

    if(StrFirstBuff != "[" || StrSecBuff != "]"){
        let IntFirstBuff:i32 = StrFirstBuff.to_string().parse().unwrap();

        let IntSecBuff:i32 = StrSecBuff.to_string().parse().unwrap();
    
        if(Token == '+'){println!("{}",AddGeneric(IntFirstBuff,IntSecBuff));}
        if(Token == '-'){println!("{}",SubGeneric(IntFirstBuff,IntSecBuff));}
        if(Token == '*'){println!("{}",MultGeneric(IntFirstBuff,IntSecBuff));}
        if(Token == '^'){println!("{}",ExpGeneric(IntFirstBuff,IntSecBuff));}
    }
}

fn SelectNumbersFromVec(FirstBuff:Vec<String>,SecBuff:Vec<String>,Token:Vec<char>,arg:String,Start: std::time::Instant){
    //Var create to store number for loop
    let mut WhereFirstBuff:usize=0;
    let mut WhereSecBuff:usize=0;
    
    //Get number of lines from file
    let mut number_of_lines=0;
    for (i,x) in arg.chars().enumerate(){
        if(x == '\n'){number_of_lines = number_of_lines + 1;}
    }
    //println!("{}",number_of_lines);

    //Create to increment number and send it into Where___Buffer to store number for loop
    let mut SecCounter:usize =0;
    let mut FirstCounter:usize =0;

    for a in 0..number_of_lines{
        //create to contain numbers
        let mut StringFirstBuff:String ="".to_string();
        let mut StringSecBuff:String ="".to_string();

        //iterate after FirstBuff to get a number(String)
        //WhereFirstBuff contain saved number from previous step
        for i in WhereFirstBuff..FirstBuff.len(){  
            FirstCounter=FirstCounter+1;
            if(FirstBuff[i] == "\n"){WhereFirstBuff = FirstCounter;break;}
            StringFirstBuff += &FirstBuff[i].clone();
        }        
        for i in WhereSecBuff..SecBuff.len(){
            SecCounter=SecCounter+1;
            if(SecBuff[i] == ";"){WhereSecBuff = SecCounter;break;}
            StringSecBuff += &SecBuff[i].clone();
        }
        ConvertToInt(StringFirstBuff,StringSecBuff,Token[a]);
    }
    println!("-------------------------");
    println!("{:?}",Start.elapsed());
}


fn GetFileContent(path:&str) -> String{
    let mut contents = fs::read_to_string(path)
    .expect("->");

    return contents.replace(" ",""); 

    //return contents.to_string();
}

fn CreateReadFile(PATH:&str){    
    let mut Icode = File::create("RCD.csl").unwrap();
    let mut Ucode = File::open("Domath.txt").unwrap();

    let mut reader = BufReader::new(&Ucode);
    let mut writer = BufWriter::new(&Icode);
    let mut Buff = String::from("");
        
    for (index, line) in reader.lines().enumerate() {
        let mut Buff = line.as_ref().unwrap();
       if Buff.replace(" ","") == ""{
            writeln!(writer,"{}","[+];");
       }
       else{
            writeln!(writer,"{}",Buff);
       }
   }
   //fs::remove_file("Domath.txt");
   //fs::rename("RCD.txt", "Domath.txt").unwrap();
}

fn main(){
    let Start = std::time::Instant::now();
    if(Path::new("RCD.csl").is_file()){fs::remove_file("RCD.csl");}
    let PATH = "Domath.txt";
    CreateReadFile(PATH);
    GetNumbers(GetFileContent("RCD.csl"),Start);
}