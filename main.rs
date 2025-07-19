use std::str::FromStr;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};

enum Commands {
  Add,
  Remove,
  UpdateStatus,
  UpdateLevel,
  UpdateDeadLine,
  See,
  SeeDate,
  SeeLevel,
  Verify,
  Help,
}

impl FromStr for Commands {

  type Err = String;
   
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "add" => Ok(Commands::Add),
      "remove" => Ok(Commands::Remove),
      "updatestatus" => Ok(Commands::UpdateStatus),
      "updatelevel" => Ok(Commands::UpdateLevel),
      "updatedeadline" => Ok(Commands::UpdateDeadLine),
      "see" => Ok(Commands::See),
      "see_sort_date" => Ok(Commands::SeeDate),
      "see_sort_level" => Ok(Commands::SeeLevel),
      "help" => Ok(Commands::Help),
      "verif" => Ok(Commands::Verify),
      _ => Err("Invalid command".to_string()),
    }
  }
}

fn is_leap(x: u16) -> bool {
  if x == 0 {
    return false;
  };
  if x % 4 == 0 {
    if x % 100 == 0 {
      if x % 400 == 0 {
        return true;
      } else {
        return false;
      };
    } else {
      return true;
    };
  } else {
    return false;
  };
}

fn is_real_date(day: u8, month: u8, year: u16) -> Result<(), String> {
  if month > 12 {
    return Err("month can not be over 12".to_string());
  }
  if month == 0 {
    return Err("month start at 1".to_string());
  }
  if day == 0 {
    return Err("day start at 1".to_string());
  }
  const NB_DAYS: [u8; 12] = [31, 29, 
                             31, 30, 
                             31, 30, 
                             31, 31, 
                             30, 31, 
                             30, 31];
  const NB_DAYS_LEAP: [u8; 12] = [31, 28, 
                                  31, 30, 
                                  31, 30, 
                                  31, 31, 
                                  30, 31, 
                                  30, 31];
  if is_leap(year)  {
    if day > NB_DAYS_LEAP[(month as usize) - 1] {
      return Err(format!("Day is higer than it can be, max is: {}", 
              NB_DAYS_LEAP[(month as usize) - 1]));
    }
  } else {
    if day > NB_DAYS[(month as usize) - 1] {
      return Err(format!("Day is higer than it can be, max is: {}", 
              NB_DAYS[(month as usize) - 1]));
    }
  }
  Ok(())
}

fn parse_deadline(cur_deadline: String) -> Result<(u8, u8, u16), String> {
  let date_vec: Vec<&str> = cur_deadline.split("-").collect();
  if date_vec.len() != 3 {
    return Err("Wrong deadline format".to_string());
  }
  if date_vec[0].chars().count() != 2 {
    return Err("The day must be 2 chars long".to_string());
  }
  if date_vec[1].chars().count() != 2 {
    return Err("The month must be 2 chars long".to_string());
  }
  let day: u8 = (date_vec[0])
      .parse::<u8>()
      .map_err(|_| "Wrong day format")?;
  let month: u8 = (date_vec[1])
      .parse::<u8>()
      .map_err(|_| "Wrong month format")?;
  let year: u16 = (date_vec[2])
      .parse::<u16>()
      .map_err(|_| "Wrong year format")?;
  is_real_date(day.clone(), month.clone(), year.clone())?;
  Ok((day, month, year))
}

fn extract_arg(x: &Vec<String>, n: usize, target: usize) -> Result<String, String> {
  match n != target {
    true => Err("Too much or not enough args, consider calling the help command".to_string()),
    false => Ok(x[target - 1].clone()),
  }
}

fn func_add(x: &String) -> Result<(), String> {
  let info_vec: Vec<String> = x.split(",").map(|s| s.to_string()).collect();
  if info_vec.len() != 4 {
    return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
  }
  let mut name: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();
  let mut deadline: String = "".to_string();
  let iter_rslt: Result<(), String>;
  iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          let iter_vec: Vec<String> = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
  iter_rslt.map_err(|e| e)?;
  if [&name, &status_string, &level_string, &deadline]
    .iter()
    .any(|s| s.is_empty()) {
    return Err("Missing one or more fields".to_string());
  }
  status_string
      .parse::<bool>()
      .map_err(|_| "Wrong format for status".to_string())?;
  level_string
      .parse::<u8>()
      .map_err(|_| "Wrong format for level".to_string())?;
  parse_deadline(deadline.clone())
      .map_err(|e| e)?;
  let newline: String = format!("name:{},status:{},level:{},deadline:{}",
                                name, 
                                status_string,
                                level_string,
                                deadline);
  let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open("tasks.txt")
      .map_err(|_| "Error appending to tasks.txt".to_string())?;
  writeln!(file, "{}", newline)
      .map_err(|_| "Error appending to tasks.txt".to_string())?;
  Ok(())
}

fn remove(x: &String) -> Result<(), String> {
  let mut has_been_found: bool = false;
  let mut file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt".to_string())?;
  let reader = BufReader::new(file);

  let mut line_vec: Vec<String> = vec![];
  let mut iter_vec: Vec<String> = vec![];
  let mut iter_rslt: Result<(), String>;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    if name != *x {
      line_vec.push(content.clone());
    } else {
      has_been_found = true;
    }
  }
  if !has_been_found {
    return Err(format!("task: {} is not present", name));
  }
  file = File::create("tasks.txt").map_err(|_| "Error updating tasks.txt".to_string())?;
  for line in line_vec {
    writeln!(file, "{}", line).map_err(|_| "Error updating tasks.txt".to_string())?;
  }
  Ok(())
}

fn update_status(x: &String, new_status: &String) -> Result<(), String> {
  new_status.parse::<bool>().map_err(|_| "Wrong status value".to_string())?;
  let mut file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt".to_string())?;
  let reader = BufReader::new(file);

  let mut line_vec: Vec<String> = vec![];
  let mut iter_vec: Vec<String> = vec![];
  let mut iter_rslt: Result<(), String>;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    if name != *x {
      line_vec.push(content.clone());
    } else {
      let up_line = format!("name:{},status:{},level:{},deadline:{}",
                            name, 
                            *new_status, 
                            level_string,
                            deadline);
      line_vec.push(up_line);
    }
  }
  file = File::create("tasks.txt").map_err(|_| "Error updating tasks.txt".to_string())?;
  for line in line_vec {
    writeln!(file, "{}", line).map_err(|_| "Error updating tasks.txt".to_string())?;
  }
  Ok(())
}

fn update_level(x: &String, new_level: &String) -> Result<(), String> {
  new_level.parse::<u8>().map_err(|_| "Wrong level value".to_string())?;
  let mut file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt".to_string())?;
  let reader = BufReader::new(file);

  let mut line_vec: Vec<String> = vec![];
  let mut iter_vec: Vec<String> = vec![];
  let mut iter_rslt: Result<(), String>;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    if name != *x {
      line_vec.push(content.clone());
    } else {
      let up_line = format!("name:{},status:{},level:{},deadline:{}",
                            name, 
                            status_string,
                            *new_level,
                            deadline);
      line_vec.push(up_line);
    }
  }
  file = File::create("tasks.txt").map_err(|_| "Error updating tasks.txt".to_string())?;
  for line in line_vec {
    writeln!(file, "{}", line).map_err(|_| "Error updating tasks.txt".to_string())?;
  }
  Ok(())
}

fn update_deadline(x: &String, new_deadline: &String) -> Result<(), String> {
  parse_deadline((*new_deadline).clone()).map_err(|e| e)?;
  let mut file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt".to_string())?;
  let reader = BufReader::new(file);

  let mut line_vec: Vec<String> = vec![];
  let mut iter_vec: Vec<String> = vec![];
  let mut iter_rslt: Result<(), String>;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    if name != *x {
      line_vec.push(content.clone());
    } else {
      let up_line = format!("name:{},status:{},level:{},deadline:{}",
                            name, 
                            status_string,
                            level_string,
                            *new_deadline);
      line_vec.push(up_line);
    }
  }
  file = File::create("tasks.txt").map_err(|_| "Error updating tasks.txt".to_string())?;
  for line in line_vec {
    writeln!(file, "{}", line).map_err(|_| "Error updating tasks.txt".to_string())?;
  }
  Ok(())
}

fn see() -> Result<(), String> {
  let file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt")?;
  let reader = BufReader::new(file);

  let mut iter_vec: Vec<String> = vec![];
  let mut iter_rslt: Result<(), String>;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  println!("     TASK NAME     |     STATUS      |     LEVEL      |     DEADLINE     ");

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    println!("{:<19}| {:<16}| {:<15}| {:<18}", 
        (&name)
        .chars()
        .take(
              if name.len() < 19 { 
                name.len() 
              } else { 
                19 
              }
                )
        .collect::<String>(), 
        (&status_string)
        .chars()
        .take(
              if status_string.len() < 16 {
                status_string.len() 
              } else { 
                16
              }
            )
        .collect::<String>(), 
        (&level_string)
            .chars()
            .take(
              if level_string.len() < 15 {
                level_string.len() 
              } else { 
                15
              }
            )
            .collect::<String>(),
        (&deadline)
            .chars()
            .take(
              if deadline.len() < 18 {
                deadline.len() 
              } else { 
                18
              }
            )
            .collect::<String>()
        );
  }
  Ok(())
}

fn see_level(x: &String) -> Result<(), String> {
  let file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt")?;
  let reader = BufReader::new(file);
  let mut iter_rslt: Result<(), String>;
  let mut iter_vec: Vec<String> = vec![];
  let mut min_date: u64 = 0;
  let mut max_date: u64 = u64::MAX;
  let mut n_max: u16 = u16::MAX;
  let mut n_cnt: u16 = 1;
  let mut is_desc: bool = true;
  let x_vec: Vec<String> = x.split(",")
                           .map(|e| e.to_string())
                           .collect();
  iter_rslt = x_vec
      .iter()
      .map(|info| {
        let info: String = info.to_string();
        iter_vec = info.split(":").map(|e| e.to_string()).collect();
        if iter_vec.len() != 2 {
          return Err("Wrong format parameters".to_string());
        }
        match iter_vec[0].as_str() {
          "lock" => {
            iter_vec = iter_vec[1]
                .split("|")
                .map(|e| e.to_string())
                .collect();
            if iter_vec.len() != 2 {
              return Err("Error in lock parameter".to_string());
            }
            let (mut a, mut b, mut c) = parse_deadline(iter_vec[0].clone())
                .map_err(|e| format!("Error in lock parameter {}", e))?;
            min_date = (a as u64) + (b as u64) * 100 + (c as u64) * 1000;
            
            (a, b, c) = parse_deadline(iter_vec[1].clone())
                .map_err(|_| "Error in lock parameter".to_string())?;
            max_date = (a as u64) + (b as u64) * 100 + (c as u64) * 10000;
            if min_date > max_date {
              return Err("The first date must be inferior to the second date".to_string());
            }
            Ok(())
          },
          "n" => {
            n_max = iter_vec[1]
                .parse::<u16>()
                .map_err(|_| "Error parsing the n max parameter".to_string())?;
            Ok(())
          },
          "order" => {
            match iter_vec[1].as_str() {
              "asc" => {
                is_desc = false;
                Ok(())
              },
              "desc" => Ok(()),
              _ => Err("The order is undefined".to_string()),
            }
          },
          _ => { return Err(format!("Parameter {} corresponds to nothing", iter_vec[1])); },
        }
      })
      .collect();
 
  iter_rslt.map_err(|e| e)?;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  let mut to_sort_vec: Vec<u8> = vec![];
  let mut to_print_vec: Vec<String> = vec![];

  println!("     TASK NAME     |     STATUS      |     LEVEL      |     DEADLINE     ");

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    let (day, month, year) = parse_deadline(deadline.clone())
        .map_err(|e| format!("In tasks.txt: {}", e))?;
    let cur_date: u64 = (day as u64) + (month as u64) * 100 + (year as u64) * 10000;
    let cur_level: u8 = level_string
        .parse::<u8>()
        .map_err(|_| format!("Bad level for task: '{}'", name))?;
    if min_date <= cur_date && max_date >= cur_date {
      to_sort_vec.push(cur_level);
      to_print_vec.push(format!("{:<19}| {:<16}| {:<15}| {:<18}", 
          (&name)
          .chars()
          .take(
                if name.len() < 19 { 
                  name.len() 
                } else { 
                  19 
                }
                  )
          .collect::<String>(), 
          (&status_string)
          .chars()
          .take(
                if status_string.len() < 16 {
                  status_string.len() 
                } else { 
                  16
                }
              )
          .collect::<String>(), 
          (&level_string)
              .chars()
              .take(
                if level_string.len() < 15 {
                  level_string.len() 
                } else { 
                  15
                }
              )
              .collect::<String>(),
          (&deadline)
              .chars()
              .take(
                if deadline.len() < 18 {
                  deadline.len() 
                } else { 
                  18
                }
              )
              .collect::<String>()
          )
      );
    }
  }
  let mut ref_indices: Vec<usize> = (0..to_sort_vec.len()).collect();
  ref_indices.sort_by_key(|&i| to_sort_vec[i]);
  if is_desc {
    ref_indices.reverse();
  }
  let sorted_to_print_vec: Vec<String> = ref_indices
      .iter()
      .map(|&i| to_print_vec[i].clone())
      .collect();
  for cur_line in &sorted_to_print_vec {
    println!("{}", cur_line);
    n_cnt += 1;
    if n_cnt > n_max {
      break
    }
  }
  Ok(())
}

fn see_date(x: &String) -> Result<(), String> {
  let file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt")?;
  let reader = BufReader::new(file);
  let mut iter_rslt: Result<(), String>;
  let mut iter_vec: Vec<String> = vec![];
  let mut min_date: u64 = 0;
  let mut max_date: u64 = u64::MAX;
  let mut n_max: u16 = u16::MAX;
  let mut n_cnt: u16 = 1;
  let mut is_desc: bool = true;
  let x_vec: Vec<String> = x.split(",")
                           .map(|e| e.to_string())
                           .collect();
  iter_rslt = x_vec
      .iter()
      .map(|info| {
        let info: String = info.to_string();
        iter_vec = info.split(":").map(|e| e.to_string()).collect();
        if iter_vec.len() != 2 {
          return Err("Wrong format parameters".to_string());
        }
        match iter_vec[0].as_str() {
          "lock" => {
            iter_vec = iter_vec[1]
                .split("|")
                .map(|e| e.to_string())
                .collect();
            if iter_vec.len() != 2 {
              return Err("Error in lock parameter".to_string());
            }
            let (mut a, mut b, mut c) = parse_deadline(iter_vec[0].clone())
                .map_err(|e| format!("Error in lock parameter {}", e))?;
            min_date = (a as u64) + (b as u64) * 100 + (c as u64) * 1000;
            
            (a, b, c) = parse_deadline(iter_vec[1].clone())
                .map_err(|_| "Error in lock parameter".to_string())?;
            max_date = (a as u64) + (b as u64) * 100 + (c as u64) * 10000;
            if min_date > max_date {
              return Err("The first date must be inferior to the second date".to_string());
            }
            Ok(())
          },
          "n" => {
            n_max = iter_vec[1]
                .parse::<u16>()
                .map_err(|_| "Error parsing the n max parameter".to_string())?;
            Ok(())
          },
          "order" => {
            match iter_vec[1].as_str() {
              "asc" => {
                is_desc = false;
                Ok(())
              },
              "desc" => Ok(()),
              _ => Err("The order is undefined".to_string()),
            }
          },
          _ => { return Err(format!("Parameter {} corresponds to nothing", iter_vec[1])); },
        }
      })
      .collect();
 
  iter_rslt.map_err(|e| e)?;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  let mut to_sort_vec: Vec<u64> = vec![];
  let mut to_print_vec: Vec<String> = vec![];

  println!("     TASK NAME     |     STATUS      |     LEVEL      |     DEADLINE     ");

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    let (day, month, year) = parse_deadline(deadline.clone())
        .map_err(|e| format!("In tasks.txt: {}", e))?;
    let cur_date: u64 = (day as u64) + (month as u64) * 100 + (year as u64) * 10000;
    if min_date <= cur_date && max_date >= cur_date {
      to_sort_vec.push(cur_date);
      to_print_vec.push(format!("{:<19}| {:<16}| {:<15}| {:<18}", 
          (&name)
          .chars()
          .take(
                if name.len() < 19 { 
                  name.len() 
                } else { 
                  19 
                }
                  )
          .collect::<String>(), 
          (&status_string)
          .chars()
          .take(
                if status_string.len() < 16 {
                  status_string.len() 
                } else { 
                  16
                }
              )
          .collect::<String>(), 
          (&level_string)
              .chars()
              .take(
                if level_string.len() < 15 {
                  level_string.len() 
                } else { 
                  15
                }
              )
              .collect::<String>(),
          (&deadline)
              .chars()
              .take(
                if deadline.len() < 18 {
                  deadline.len() 
                } else { 
                  18
                }
              )
              .collect::<String>()
          )
      );
    }
  }
  let mut ref_indices: Vec<usize> = (0..to_sort_vec.len()).collect();
  ref_indices.sort_by_key(|&i| to_sort_vec[i]);
  if is_desc {
    ref_indices.reverse();
  }
  let sorted_to_print_vec: Vec<String> = ref_indices
      .iter()
      .map(|&i| to_print_vec[i].clone())
      .collect();
  for cur_line in &sorted_to_print_vec {
    println!("{}", cur_line);
    n_cnt += 1;
    if n_cnt > n_max {
      break
    }
  }
  Ok(())
}

fn verify() -> Result<(), String> {
  let file = File::open("tasks.txt")
      .map_err(|_| "Unable to open tasks.txt".to_string())?;
  let reader = BufReader::new(file);

  let mut alrd_names: Vec<String> = vec![];

  let mut iter_vec: Vec<String> = vec![];
  let mut iter_rslt: Result<(), String>;

  let mut name: String = "".to_string();
  let mut deadline: String = "".to_string();
  let mut status_string: String = "".to_string();
  let mut level_string: String = "".to_string();

  for line in reader.lines() {
    let content = line.map_err(|_| "Error reading file".to_string())?;
    let info_vec: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    if info_vec.len() != 4 {
      return Err("Bad number of informations, task fields: name, status, level, deadline".to_string());
    }
    iter_rslt = info_vec
        .iter()
        .map(|info| {
          let info: String = info.to_string();
          iter_vec = info.split(":").map(|el| el.to_string() ).collect();
          if iter_vec.len() != 2 {
            return Err("Bad value definition".to_string());
          }
          match iter_vec[0].as_str() {
            "name" => {
                if iter_vec[1].is_empty() {
                  return Err("name lacks a value".to_string());
                }
                name = iter_vec[1].clone();
                Ok(()) 
              },
            "status" => {
                if iter_vec[1].is_empty() {
                  return Err("status lacks a value".to_string());
                }
                status_string = iter_vec[1].clone();
                Ok(())
              },
            "level" => {
                if iter_vec[1].is_empty() {
                  return Err("level lacks a value".to_string());
                }
                level_string = iter_vec[1].clone();
                Ok(())
              },
            "deadline" => {
                if iter_vec[1].is_empty() {
                  return Err("deadline lacks a value".to_string());
                }
                deadline = iter_vec[1].clone();
                Ok(()) 
              },
            _ => Err("Wrong field, task fields: name, status, level, deadline".to_string()),
          }
        })
        .collect();
    iter_rslt.map_err(|e| e)?;
    if [&name, &status_string, &level_string, &deadline]
      .iter()
      .any(|s| s.is_empty()) {
      return Err("Missing one or more fields".to_string());
    }
    for alrd_name in &alrd_names {
      if name == *alrd_name {
        return Err(format!("A task has already the same name: '{}'", name));
      }
    }
    alrd_names.push(name.clone());
    status_string
        .parse::<bool>()
        .map_err(|_| "Status format is wrong".to_string())?;
    level_string
        .parse::<u8>()
        .map_err(|_| "Level format is wrong".to_string())?;
    parse_deadline(deadline.clone())
        .map_err(|_| "Deadline format is wrong".to_string())?;
  }
  Ok(()) 
}

fn help() {
  println!("ADD TASK: taskname 'name:actual_name,status:false_or_true,level:u8,deadline:day-month-year'");
  println!("REMOVE TASK: remove actual_name");
  println!("UPDATE STATUS: updatestatus false_or_true");
  println!("UPDATE LEVEL: updatelevel u8");
  println!("UPDATE DEADLINE: updatedeadline 'day-month-year'");
  println!("SEE ALL TASKS: see");
  println!("SEE DATE SORTED METHODS: see_sort_date 'lock:min_date|max_date,order:asc_or_desc,n:number_of_tasks_to_print'");
  println!("SEE LEVEL SORTED METHODS: see_sort_date 'lock:min_date|max_date,order:asc_or_desc,n:number_of_tasks_to_print'");
  println!("HELP: help");
  println!("NOTE: you can vary the order of parameters, like name,deadline,status,level is also fine");
  println!("NOTE: date system is Gregorian");
}

fn main () -> Result<(), String> {
  let args_v: Vec<String> = env::args().collect();
  let n: usize = args_v.len();
  if n < 2 {
    return Err("No command was provided".to_string());
  }
  
  let cmd: Commands = args_v[1].parse::<Commands>()?;

  match cmd {
    Commands::Add => {
      extract_arg(&args_v, n, 3).map_err(|err| err)?;
      func_add(&args_v[2]).map_err(|err| err)?;
    },
    Commands::Remove => {
      extract_arg(&args_v, n, 3).map_err(|err| err)?;
      remove(&args_v[2]).map_err(|err| err)?;
    },
    Commands::UpdateStatus => {
      extract_arg(&args_v, n, 4).map_err(|err| err)?;
      update_status(&args_v[2], &args_v[3]).map_err(|err| err)?;
    },
    Commands::UpdateLevel => {
      extract_arg(&args_v, n, 4).map_err(|err| err)?;
      update_level(&args_v[2], &args_v[3]).map_err(|err| err)?;
    },
    Commands::UpdateDeadLine => {
      extract_arg(&args_v, n, 4).map_err(|err| err)?;
      update_deadline(&args_v[2], &args_v[3]).map_err(|err| err)?;
    },
    Commands::See => {
      extract_arg(&args_v, n, 2).map_err(|err| err)?;
      see()?;
    },
    Commands::SeeDate => {
      extract_arg(&args_v, n, 3).map_err(|err| err)?;
      see_date(&args_v[2])?;
    },
    Commands::SeeLevel => {
      extract_arg(&args_v, n, 3).map_err(|err| err)?;
      see_level(&args_v[2])?;
    },
    Commands::Verify => {
      extract_arg(&args_v, n, 2).map_err(|err| err)?;
      verify()?;
    },
    Commands::Help => {
      extract_arg(&args_v, n, 2).map_err(|err| err)?;
      help();
    },
  };

  return Ok(());
}



