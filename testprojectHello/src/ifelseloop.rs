pub fn main(){
    println!("Normal Loop");
    generatestarsloop(10);
    println!("For Loop");
    generatestarsforloop(10);
    println!("While Loop");
    generatestarswhileloop(10);
}

pub fn generatestarsloop(rows: i32){
    let mut rowCounter:i32=0;
    'rowwise: loop{
        let mut rowString:String=String::new();
        if rowCounter>rows{
            break 'rowwise;
        }
        else{
            let mut columnCounter: i32=0;
            'columnwise: loop{
                if columnCounter>rowCounter{
                    break 'columnwise;
                    
                }   
                else{
                    rowString.push_str("*");
                    // println!("Column Count{}",columnCounter);
                    columnCounter+=1;
                }
            }
        }
        rowCounter+=1;
        println!("{}",rowString);

    }
}

pub fn generatestarsforloop(rows: i32){
    'rowise: for i in 1..=rows{
        let mut string:String=String::new();
        'columnwise: for j in 1..=i{

            string.push_str("*");
        }
        println!("{}",string);
    }
}

pub fn generatestarswhileloop(rows: i32){
    let mut counter:i32=0;

    'rowwise: while counter<=rows{
        let mut columncounter=0;
        let mut row:String=String::new();
        'columnwise: while columncounter<=counter{
            row.push_str("*");
            columncounter+=1;
        }
        println!("{}",row);
        counter+=1;
    }
}