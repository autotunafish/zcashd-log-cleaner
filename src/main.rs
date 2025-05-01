use ::std::fs;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    //Name of the dirty log file
    let pathin = Path::new("../../zcashd.log");
    //Name of the clean log file
    let pathout = Path::new("../../zcashd2.log");

    //These hold the cleaned string before it's written and so must persist till just after it's loop
    let mut _newrows4 = String::new();
    let mut catstring = String::new();
    let mut batstring = String::new();

    //Set if the zcashd2.log already exists
    let mut exibool: bool = false;

    //Holds the line count of the respective logs
    let mut _lc_dirty = 0;
    let mut _lc_clean = 0;

    //Duration for file check the interval
    let nap = Duration::from_secs(5);

    //Check if the files are present
    if pathout.exists() {
        //If so, skip the DB initial full-build
        exibool = true;
    } else {
        //Creates the file on the initial run
        fs::write(pathout, "").expect("Unable to create zcashd2.log");
    }

    //Will panic if zcashd.log is not present
    let _file = File::open(pathin)
        .expect("*****Unable to open zcashd.log. Is a zcashd.log present in this directory?******");

    let _file2 = File::open(pathout).expect(
        "*****Unable to open zcashd2.log. Is a zcashd2.log present in this directory?******",
    );

    //Read in the log content as text
    let mut rows = fs::read_to_string(pathin).unwrap();
    //println!("ROWS::\n{}", &rows);

    let mut _rows2 = fs::read_to_string(pathout).unwrap();
    //println!("ROWS2::\n{}", &_rows2);

    //Build the entire DB new if none was found
    if exibool == false {
        //First, the zcashd.log 'rows'
        //The text patterns to be removed
        let target = String::from("#033[0m");
        let target1 = String::from("#033[1m");
        let target2 = String::from("#033[2m");
        let target3 = String::from("#033[3m");

        //Each line of 'rows' is cleaned of all patterns and then concatenated to into 'zcashd2.log'
        for i in rows.lines() {
            let newrows = String::from(i).clone();
            let newrows1 = newrows.replace(&target, "");
            let new_rows2 = newrows1.replace(&target1, "");
            let newrows3 = new_rows2.replace(&target2, "");
            _newrows4 = newrows3.replace(&target3, "");

            //println!("newrow4 1:: \n{}", &_newrows4);

            catstring.push_str(&_newrows4);
            catstring.push('\n');
        }

        //println!("catstring \n{}", &catstring);

        catstring.pop();
        fs::write(pathout, catstring).unwrap();
    }

    //
    //If the DB is present then get the current linecounts of the respective files and compare

    //Ensure that any and all \n are removed (trim() was being difficult)
    loop {
        rows = fs::read_to_string(pathin).unwrap();
        loop {
            let popped = rows.pop().unwrap();
            if popped == '\n' { continue } else { break }
        }

        _lc_dirty = rows.lines().count();

        _rows2 = fs::read_to_string(pathout).unwrap();
        loop {
            let popped = _rows2.pop().unwrap();
            if popped == '\n' { continue } else { break }
        }

        _lc_clean = _rows2.lines().count();

        //println!("LINECOUNTS------D--{}---C--{}---", &_lc_dirty, &_lc_clean);

        //This value determines the appx depth of search by subtraction
        if (_lc_dirty - 1) < _lc_clean {
            thread::sleep(nap);
            continue;
        }
        let mut diff: i32 = (_lc_dirty - _lc_clean).try_into().unwrap();

        //println!("diff:: {}", diff);
        //println!("rows::  \n{}", &rows);
        //println!("_rows2:: \n{}", &_rows2);

        //Last lines of each file for comparison. The initial last line is a '\n' so pop first
        rows.pop();
        _rows2.pop();
        let mut _dirtylastline = rows.lines().last().clone().unwrap();
        let mut _cleanlastline = _rows2.lines().last().clone().unwrap();

        //println!("_dirtylastline  \n{}", &_dirtylastline);
        //println!("_cleanlastline \n{}", &_cleanlastline);

        //

        //Extract the tailing lines that differ from the two files according to diff
        let mut varistring = String::new();
        loop {
            if diff == 0 {
                break;
            }
            let seekreturn = rows.pop().unwrap();
            if seekreturn == '\n' {
                diff -= 1;
                varistring.insert(0, seekreturn);
            } else {
                varistring.insert(0, seekreturn);
            }
        }

        //println!("varistring::\n{}", &varistring);

        //Clean the strings
        let target = String::from("#033[0m");
        let target1 = String::from("#033[1m");
        let target2 = String::from("#033[2m");
        let target3 = String::from("#033[3m");

        //Each line of 'rows' is cleaned of all patterns and then concatenated to into 'zcashd2.log'
        for i in varistring.lines() {
            let _newrows = String::from(i).clone();
            let _newrows1 = _newrows.replace(&target, "");
            let new_rows2 = _newrows1.replace(&target1, "");
            let newrows3 = new_rows2.replace(&target2, "");
            _newrows4 = newrows3.replace(&target3, "");

            //println!("newrow4 1:: \n{}", &_newrows4);

            batstring.push_str(&_newrows4);
            batstring.push('\n');
        }

        //Format the cleaned strings to the original clean text
        varistring = batstring.clone();
        //println!("varistring:: \n{}", &varistring);

        let mut nxtline = String::new();

        for i in varistring.lines() {
            nxtline.push_str(i);
            nxtline.push('\n');
        }

        //println!("nxtline---------\n{}", &nxtline);

        //Get the original text again (just in case I guess) and cat the new stuff
        _rows2 = fs::read_to_string(pathout).unwrap();
        _rows2.pop();
        _rows2.push_str(&nxtline);

        //println!("row2 fin--------------\n{}", &_rows2);

        //Write it to the clean log
        fs::write(pathout, &_rows2).unwrap();

        //clear all strings to be sure
        batstring.clear();
        varistring.clear();
        _rows2.clear();
        _rows2.clear();
    }
}
