use colored::*;
use enum_iterator::IntoEnumIterator;
use std::process;

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq)]
enum Block {
  O = 1,   // 四角ぽいやつ
  T = 2,   // Tっぽい形のやつ
  U = 3,   // Uっぽい形のやつ
  X = 4,   // 十字ぽいやつ
  W = 5,   // Wっぽいやつ
  P = 6,   // Pぽいやつ
  S = 7,   // Sっぽいやつ
  L = 8,   // Lっぽいやつ
  V = 9,   // Vっぽいやつ(Lを傾けたみたいな)
  I = 10,  // Iっぽいやつ
  TO = 11, // "ト" みたいなやつ
  SU = 12, // "ス" みたいなやつ
  LS = 13, // 長いSみたいなやつ
}

pub fn solve() {
  let frame: [[i8; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
  ];

  let rest_blocks = [
    Block::O,
    Block::T,
    Block::U,
    Block::X,
    Block::W,
    Block::P,
    Block::S,
    Block::L,
    Block::V,
    Block::I,
    Block::TO,
    Block::SU,
    Block::LS,
  ];

  dive(frame, &rest_blocks, &[]);
}

fn survey_and_fill(frame: &mut [[i8; 8]; 8], x: usize, y: usize) -> i8 {
  // 隣接するすべてのエリアにダミー値を入れつつカウントする
  let mut count = 1;

  frame[y][x] = 99;

  // 右
  if x <= 6 && frame[y][x + 1] == 0 {
    count += survey_and_fill(frame, x + 1, y);
  }
  // 下
  if y <= 6 && frame[y + 1][x] == 0 {
    count += survey_and_fill(frame, x, y + 1);
  }
  // 左
  if x >= 1 && frame[y][x - 1] == 0 {
    count += survey_and_fill(frame, x - 1, y);
  }
  // 上
  if y >= 1 && frame[y - 1][x] == 0 {
    count += survey_and_fill(frame, x, y - 1);
  }

  return count;
}

fn has_clear_possibility(frame: [[i8; 8]; 8]) -> bool {
  let mut new_frame = frame.clone();

  for y in 0..8 {
    for x in 0..8 {
      // 空白なら調査
      if new_frame[y][x] == 0 {
        // この空白を起点とする面積が一定のもの以外ならパターンとして不正
        let space_count = survey_and_fill(&mut new_frame, x, y);
        if space_count % 5 != 0 && space_count % 5 != 4 {
          // println!("これはだめです");
          // print_frame(frame);
          return false;
        }
      }
    }
  }

  return true;
}

fn fill_block_pattern(
  frame: [[i8; 8]; 8],
  block: Block,
  pattern: &Vec<Vec<i8>>,
  putx: usize,
  puty: usize,
) -> (bool, [[i8; 8]; 8]) {
  let mut new_frame = frame.clone();

  let height = pattern.len();
  let width = pattern[0].len();

  for y in 0..height {
    let line = &pattern[y];
    for x in 0..width {
      let data = [line[x], new_frame[puty + y][putx + x]];
      match data {
        [1, 0] => {
          new_frame[puty + y][putx + x] = block as i8;
        }
        [1, 1..=13] => {
          return (false, frame);
        }
        _ => (),
      }
    }
  }

  // 置いた結果、クリア不可能なパターンならスキップ
  if !has_clear_possibility(new_frame) {
    return (false, frame);
  }

  return (true, new_frame);
}

fn put_block_pattern(
  frame: [[i8; 8]; 8],
  block: Block,
  pattern: &Vec<Vec<i8>>,
) -> (bool, [[i8; 8]; 8]) {
  let height = pattern.len();
  let width = pattern[0].len();
  for y in 0..=(8 - height) {
    for x in 0..=(8 - width) {
      match fill_block_pattern(frame, block, pattern, x, y) {
        (true, new_frame) => {
          return (true, new_frame);
        }
        (false, _) => (),
      }
    }
  }

  return (false, frame);
}

fn dive(frame: [[i8; 8]; 8], rest_blocks: &[Block], tried_stack: &[Block]) {
  // 次のブロックを得る
  let block = rest_blocks[0];

  // 全パターン網羅する
  let block_patterns = get_block_patterns(block);

  for pattern in block_patterns.iter() {
    match put_block_pattern(frame, block, pattern) {
      (true, new_frame) => {
        // 今置いたのが最後のブロックだった場合は優勝
        if rest_blocks.len() == 1 {
          println!("優勝 {:?}", new_frame);
          print_frame(frame);
          // process::exit(0);
          return;
        }

        // 次のブロックへ行ってみる
        dive(new_frame, &rest_blocks[1..rest_blocks.len()], tried_stack);
      }
      (false, _) => (),
    }
  }

  // だめだったので、いまのブロックを最後に持っていってやり直す
  // ただし、一周した場合にはやり直さない
  if !tried_stack.contains(&block) {
    let new_rest_blocks = [&rest_blocks[1..rest_blocks.len()], &[block]].concat();
    let new_tried_stack = [tried_stack, &[block]].concat();
    dive(frame, &new_rest_blocks, &new_tried_stack);
  }
}

fn print_frame(frame: [[i8; 8]; 8]) {
  for y in 0..8 {
    for x in 0..8 {
      print!(
        "{}",
        match frame[y][x] {
          1 => {
            "■".red()
          }
          2 => {
            "■".blue()
          }
          3 => {
            "■".green()
          }
          4 => {
            "■".cyan()
          }
          5 => {
            "■".yellow()
          }
          6 => {
            "■".purple()
          }
          7 => {
            "■".truecolor(255, 255, 136)
          }
          8 => {
            "■".truecolor(200, 255, 136)
          }
          9 => {
            "■".truecolor(199, 20, 136)
          }
          10 => {
            "■".truecolor(10, 25, 136)
          }
          11 => {
            "■".truecolor(255, 30, 136)
          }
          12 => {
            "■".truecolor(23, 255, 255)
          }
          13 => {
            "■".truecolor(23, 170, 49)
          }
          99 => {
            "■".white()
          }
          _ => {
            "■".black()
          }
        }
      );
    }
    println!("");
  }
  println!("");
}

#[rustfmt::skip]
fn get_block_patterns(block: Block) -> Vec<Vec<Vec<i8>>> {
  match block {
    Block::T => vec![
      vec![
        [1,1,1].to_vec(),
        [0,1,0].to_vec(),
        [0,1,0].to_vec(),
      ],
      vec![
        [0,0,1].to_vec(),
        [1,1,1].to_vec(),
        [0,0,1].to_vec(),
      ],
      vec![
        [0,1,0].to_vec(),
        [0,1,0].to_vec(),
        [1,1,1].to_vec(),
      ],
      vec![
        [1,0,0].to_vec(),
        [1,1,1].to_vec(),
        [1,0,0].to_vec(),
      ],
    ],
    Block::U => vec![
      vec![
        [1,0,1].to_vec(),
        [1,1,1].to_vec(),
      ],
      vec![
        [1,1].to_vec(),
        [1,0].to_vec(),
        [1,1].to_vec(),
      ],
      vec![
        [1,1,1].to_vec(),
        [1,0,1].to_vec(),
      ],
      vec![
        [1,1].to_vec(),
        [0,1].to_vec(),
        [1,1].to_vec(),
      ],
    ],
    Block::X => vec![
      vec![
        [0,1,0].to_vec(),
        [1,1,1].to_vec(),
        [0,1,0].to_vec(),
      ]
    ],
    Block::O => vec![
      vec![
        [1, 1].to_vec(),
        [1, 1].to_vec()
      ]
    ],
    Block::W => vec![
      vec![
        [1,0,0].to_vec(),
        [1,1,0].to_vec(),
        [0,1,1].to_vec(),
      ],
      vec![
        [0,1,1].to_vec(),
        [1,1,0].to_vec(),
        [1,0,0].to_vec(),
      ],
      vec![
        [1,1,0].to_vec(),
        [0,1,1].to_vec(),
        [0,0,1].to_vec(),
      ],
      vec![
        [0,0,1].to_vec(),
        [0,1,1].to_vec(),
        [1,1,0].to_vec(),
      ],
    ],
    Block::P => vec![
      vec![
        [1,1].to_vec(),
        [1,1].to_vec(),
        [1,0].to_vec(),
      ],
      vec![
        [1,1].to_vec(),
        [1,1].to_vec(),
        [0,1].to_vec(),
      ],
      vec![
        [1,1,1].to_vec(),
        [0,1,1].to_vec(),
      ],
      vec![
        [0,1,1].to_vec(),
        [1,1,1].to_vec(),
      ],
      vec![
        [0,1].to_vec(),
        [1,1].to_vec(),
        [1,1].to_vec(),
      ],
      vec![
        [1,0].to_vec(),
        [1,1].to_vec(),
        [1,1].to_vec(),
      ],
      vec![
        [1,1,0].to_vec(),
        [1,1,1].to_vec(),
      ],
      vec![
        [1,1,1].to_vec(),
        [1,1,0].to_vec(),
      ],
    ],
    Block::S => vec![
      vec![
        [0,1,1].to_vec(),
        [0,1,0].to_vec(),
        [1,1,0].to_vec(),
      ],
      vec![
        [1,1,0].to_vec(),
        [0,1,0].to_vec(),
        [0,1,1].to_vec(),
      ],
      vec![
        [1,0,0].to_vec(),
        [1,1,1].to_vec(),
        [0,0,1].to_vec(),
      ],
      vec![
        [0,0,1].to_vec(),
        [1,1,1].to_vec(),
        [1,0,0].to_vec(),
      ],
    ],
    Block::L => vec![
      vec![
        [1,0].to_vec(),
        [1,0].to_vec(),
        [1,0].to_vec(),
        [1,1].to_vec(),
      ],
      vec![
        [0,1].to_vec(),
        [0,1].to_vec(),
        [0,1].to_vec(),
        [1,1].to_vec(),
      ],
      vec![
        [1,1,1,1].to_vec(),
        [1,0,0,0].to_vec(),
      ],
      vec![
        [1,0,0,0].to_vec(),
        [1,1,1,1].to_vec(),
      ],
      vec![
        [1,1].to_vec(),
        [0,1].to_vec(),
        [0,1].to_vec(),
        [0,1].to_vec(),
      ],
      vec![
        [1,1].to_vec(),
        [1,0].to_vec(),
        [1,0].to_vec(),
        [1,0].to_vec(),
      ],
      vec![
        [0,0,0,1].to_vec(),
        [1,1,1,1].to_vec(),
      ],
      vec![
        [1,1,1,1].to_vec(),
        [0,0,0,1].to_vec(),
      ],
    ],
    Block::V => vec![
      vec![
        [1,1,1].to_vec(),
        [0,0,1].to_vec(),
        [0,0,1].to_vec(),
      ],
      vec![
        [0,0,1].to_vec(),
        [0,0,1].to_vec(),
        [1,1,1].to_vec(),
      ],
      vec![
        [1,0,0].to_vec(),
        [1,0,0].to_vec(),
        [1,1,1].to_vec(),
      ],
      vec![
        [1,1,1].to_vec(),
        [1,0,0].to_vec(),
        [1,0,0].to_vec(),
      ],
    ],
    Block::I => vec![
      vec![
        [1,1,1,1,1].to_vec()
      ],
      vec![
        [1].to_vec(),
        [1].to_vec(),
        [1].to_vec(),
        [1].to_vec(),
        [1].to_vec(),
      ],
    ],
    Block::TO => vec![
      vec![
        [1,0].to_vec(),
        [1,1].to_vec(),
        [1,0].to_vec(),
        [1,0].to_vec(),
      ],
      vec![
        [0,1].to_vec(),
        [1,1].to_vec(),
        [0,1].to_vec(),
        [0,1].to_vec(),
      ],
      vec![
        [1,1,1,1].to_vec(),
        [0,0,1,0].to_vec()
      ],
      vec![
        [1,1,1,1].to_vec(),
        [0,1,0,0].to_vec()
      ],
      vec![
        [1,0].to_vec(),
        [1,0].to_vec(),
        [1,1].to_vec(),
        [1,0].to_vec(),
      ],
      vec![
        [0,1].to_vec(),
        [0,1].to_vec(),
        [1,1].to_vec(),
        [0,1].to_vec(),
      ],
      vec![
        [0,1,0,0].to_vec(),
        [1,1,1,1].to_vec(),
      ],
      vec![
        [0,0,1,0].to_vec(),
        [1,1,1,1].to_vec(),
      ],
    ],
    Block::SU => vec![
      vec![
        [1,1,0].to_vec(),
        [0,1,1].to_vec(),
        [0,1,0].to_vec(),
      ],
      vec![
        [0,1,1].to_vec(),
        [1,1,0].to_vec(),
        [0,1,0].to_vec(),
      ],
      vec![
        [0,0,1].to_vec(),
        [1,1,1].to_vec(),
        [0,1,0].to_vec(),
      ],
      vec![
        [1,0,0].to_vec(),
        [1,1,1].to_vec(),
        [0,1,0].to_vec(),
      ],
      vec![
        [0,1,0].to_vec(),
        [1,1,0].to_vec(),
        [0,1,1].to_vec(),
      ],
      vec![
        [0,1,0].to_vec(),
        [0,1,1].to_vec(),
        [1,1,0].to_vec(),
      ],
      vec![
        [0,1,0].to_vec(),
        [1,1,1].to_vec(),
        [1,0,0].to_vec(),
      ],
      vec![
        [0,1,0].to_vec(),
        [1,1,1].to_vec(),
        [0,0,1].to_vec(),
      ],
    ],
    Block::LS => vec![
      vec![
        [1,0].to_vec(),
        [1,0].to_vec(),
        [1,1].to_vec(),
        [0,1].to_vec(),
      ],
      vec![
        [0,1].to_vec(),
        [0,1].to_vec(),
        [1,1].to_vec(),
        [1,0].to_vec(),
      ],
      vec![
        [0,1,1,1].to_vec(),
        [1,1,0,0].to_vec()
      ],
      vec![
        [1,1,0,0].to_vec(),
        [0,1,1,1].to_vec()
      ],
      vec![
        [1,0].to_vec(),
        [1,1].to_vec(),
        [0,1].to_vec(),
        [0,1].to_vec(),
      ],
      vec![
        [0,1].to_vec(),
        [1,1].to_vec(),
        [1,0].to_vec(),
        [1,0].to_vec(),
      ],
      vec![
        [0,0,1,1].to_vec(),
        [1,1,1,0].to_vec()
      ],
      vec![
        [1,1,1,0].to_vec(),
        [0,0,1,1].to_vec()
      ],
    ],
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dump_block_patterns() {
    for block in Block::into_enum_iter() {
      let block_patterns = get_block_patterns(block);

      for block_pattern in block_patterns {
        for line in block_pattern {
          for cell in line.iter() {
            if *cell == 1 {
              print!("■")
            } else {
              print!("□")
            }
          }
          println!("")
        }
        println!("")
      }
    }
  }
}
