use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, Font, Image, Rect, Text, TextFragment},
    input::keyboard::{KeyCode, KeyMods},
    nalgebra as na, Context, GameResult,
};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

// 游戏常量
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const MINER_WIDTH: f32 = 60.0;
const MINER_HEIGHT: f32 = 40.0;
const HOOK_LENGTH: f32 = 200.0;
const HOOK_SPEED: f32 = 5.0;
const ITEM_SIZE: f32 = 30.0;
const GAME_DURATION: Duration = Duration::from_secs(60); // 1分钟游戏时间

// 物品类型
#[derive(Debug, Clone, Copy, PartialEq)]
enum ItemType {
    Gold,
    Silver,
    Diamond,
    Rock,
}

// 物品结构体
#[derive(Debug, Clone)]
struct Item {
    item_type: ItemType,
    position: na::Point2<f32>,
    collected: bool,
}

impl Item {
    // 创建新物品
    fn new(item_type: ItemType, x: f32, y: f32) -> Self {
        Item {
            item_type,
            position: na::Point2::new(x, y),
            collected: false,
        }
    }

    // 获取物品价值
    fn value(&self) -> i32 {
        match self.item_type {
            ItemType::Gold => 100,
            ItemType::Silver => 50,
            ItemType::Diamond => 200,
            ItemType::Rock => 10,
        }
    }

    // 获取物品颜色
    fn color(&self) -> Color {
        match self.item_type {
            ItemType::Gold => Color::new(1.0, 0.84, 0.0, 1.0), // 金色
            ItemType::Silver => Color::new(0.75, 0.75, 0.75, 1.0), // 银色
            ItemType::Diamond => Color::new(0.0, 1.0, 1.0, 1.0), // 钻石蓝
            ItemType::Rock => Color::new(0.5, 0.5, 0.5, 1.0), // 灰色
        }
    }

    // 获取物品大小
    fn size(&self) -> f32 {
        match self.item_type {
            ItemType::Rock => ITEM_SIZE * 1.5, // 石头更大一些
            _ => ITEM_SIZE,
        }
    }
}

// 钩子状态
#[derive(Debug, PartialEq)]
enum HookState {
    Idle,
    Thrown,
    Retracting,
}

// 钩子结构体
#[derive(Debug)]
struct Hook {
    position: na::Point2<f32>,
    angle: f32,
    length: f32,
    state: HookState,
    attached_item: Option<usize>, // 附着的物品索引
}

impl Hook {
    // 创建新钩子
    fn new(x: f32, y: f32) -> Self {
        Hook {
            position: na::Point2::new(x, y),
            angle: std::f32::consts::PI / 2.0, // 初始角度向下
            length: 0.0,
            state: HookState::Idle,
            attached_item: None,
        }
    }

    // 更新钩子位置
    fn update(&mut self, dt: f32) {
        match self.state {
            HookState::Idle => {
                // 闲置状态，钩子在矿工位置
                self.length = 0.0;
            }
            HookState::Thrown => {
                // 抛出状态，钩子向外延伸
                self.length += HOOK_SPEED;
                if self.length >= HOOK_LENGTH {
                    self.state = HookState::Retracting;
                }
            }
            HookState::Retracting => {
                // 收回状态，钩子向内收缩
                self.length -= HOOK_SPEED;
                if self.length <= 0.0 {
                    self.length = 0.0;
                    self.state = HookState::Idle;
                    self.attached_item = None; // 收回时释放物品
                }
            }
        }

        // 计算钩子位置
        self.position.x = self.angle.cos() * self.length;
        self.position.y = self.angle.sin() * self.length;
    }

    // 发射钩子
    fn throw(&mut self, angle: f32) {
        if self.state == HookState::Idle {
            self.angle = angle;
            self.state = HookState::Thrown;
            self.length = 0.0;
            self.attached_item = None;
        }
    }

    // 检查是否碰撞到物品
    fn check_collision(&mut self, items: &mut [Item]) {
        if self.state != HookState::Thrown || self.attached_item.is_some() {
            return;
        }

        let hook_x = self.position.x;
        let hook_y = self.position.y;

        for (i, item) in items.iter_mut().enumerate() {
            if !item.collected {
                let item_x = item.position.x;
                let item_y = item.position.y;
                let item_size = item.size() / 2.0;

                // 简单的矩形碰撞检测
                if (hook_x - item_x).abs() < item_size && (hook_y - item_y).abs() < item_size {
                    self.attached_item = Some(i);
                    item.collected = true;
                    self.state = HookState::Retracting;
                    break;
                }
            }
        }
    }
}

// 矿工结构体
#[derive(Debug)]
struct Miner {
    position: na::Point2<f32>,
    width: f32,
    height: f32,
}

impl Miner {
    // 创建新矿工
    fn new(x: f32, y: f32) -> Self {
        Miner {
            position: na::Point2::new(x, y),
            width: MINER_WIDTH,
            height: MINER_HEIGHT,
        }
    }

    // 移动矿工
    fn move_left(&mut mut) {
        if self.position.x > self.width / 2.0 {
            self.position.x -= 5.0;
        }
    }

    // 移动矿工
    fn move_right(&mut self) {
        if self.position.x < SCREEN_WIDTH - self.width / 2.0 {
            self.position.x += 5.0;
        }
    }
}

// 游戏状态
struct GameState {
    miner: Miner,
    hook: Hook,
    items: Vec<Item>,
    score: i32,
    start_time: Instant,
    game_over: bool,
}

impl GameState {
    // 创建新游戏状态
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let miner = Miner::new(SCREEN_WIDTH / 2.0, 50.0);
        let hook = Hook::new(miner.position.x, miner.position.y);
        let mut items = Vec::new();

        // 生成随机物品
        let mut rng = thread_rng();
        for _ in 0..20 {
            let item_type = match rng.gen_range(0..10) {
                0..=4 => ItemType::Gold,
                5..=7 => ItemType::Silver,
                8 => ItemType::Diamond,
                9 => ItemType::Rock,
                _ => unreachable!(),
            };

            let x = rng.gen_range(ITEM_SIZE..SCREEN_WIDTH - ITEM_SIZE);
            let y = rng.gen_range(100.0..SCREEN_HEIGHT - ITEM_SIZE);

            items.push(Item::new(item_type, x, y));
        }

        Ok(GameState {
            miner,
            hook,
            items,
            score: 0,
            start_time: Instant::now(),
            game_over: false,
        })
    }

    // 更新游戏状态
    fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        // 检查游戏是否结束
        if Instant::now() - self.start_time >= GAME_DURATION {
            self.game_over = true;
            return;
        }

        // 更新钩子
        self.hook.update(dt);

        // 检查钩子与物品的碰撞
        self.hook.check_collision(&mut self.items);

        // 如果钩子收回且有附着的物品，增加分数
        if self.hook.state == HookState::Idle && self.hook.attached_item.is_some() {
            if let Some(item_idx) = self.hook.attached_item {
                if item_idx < self.items.len() {
                    let item = &self.items[item_idx];
                    self.score += item.value();
                }
                self.hook.attached_item = None;
            }
        }

        // 更新钩子的起始位置为矿工位置
        self.hook.position.x = self.miner.position.x + self.hook.angle.cos() * self.hook.length;
        self.hook.position.y = self.miner.position.y + self.hook.angle.sin() * self.hook.length;
    }

    // 绘制游戏
    fn draw(&mut self, ctx: &mut Context, graphics: &mut graphics::GraphicsContext) -> GameResult {
        graphics::clear(ctx, Color::new(0.0, 0.2, 0.4, 1.0)); // 深蓝色背景

        // 绘制矿工
        let miner_rect = Rect::new(
            self.miner.position.x - self.miner.width / 2.0,
            self.miner.position.y - self.miner.height / 2.0,
            self.miner.width,
            self.miner.height,
        );
        graphics::rectangle(
            ctx,
            graphics::DrawParam::default().dest(miner_rect.point()),
            &miner_rect,
            Color::new(0.8, 0.5, 0.3, 1.0), // 棕色矿工
        )?;

        // 绘制钩子
        if self.hook.length > 0.0 {
            let start = na::Point2::new(self.miner.position.x, self.miner.position.y);
            let end = na::Point2::new(self.hook.position.x, self.hook.position.y);
            
            // 绘制绳子
            graphics::line(
                ctx,
                graphics::DrawParam::default(),
                &[start, end],
                2.0,
                Color::new(0.5, 0.3, 0.1, 1.0), // 棕色绳子
            )?;

            // 绘制钩子
            let hook_rect = Rect::new(
                end.x - 5.0,
                end.y - 5.0,
                10.0,
                10.0,
            );
            graphics::rectangle(
                ctx,
                graphics::DrawParam::default().dest(hook_rect.point()),
                &hook_rect,
                Color::new(0.7, 0.7, 0.7, 1.0), // 灰色钩子
            )?;

            // 如果钩子附着了物品，绘制物品
            if let Some(item_idx) = self.hook.attached_item {
                if item_idx < self.items.len() {
                    let item = &self.items[item_idx];
                    let item_size = item.size();
                    let item_rect = Rect::new(
                        end.x - item_size / 2.0,
                        end.y - item_size / 2.0,
                        item_size,
                        item_size,
                    );
                    graphics::rectangle(
                        ctx,
                        graphics::DrawParam::default().dest(item_rect.point()),
                        &item_rect,
                        item.color(),
                    )?;
                }
            }
        }

        // 绘制物品
        for item in &self.items {
            if !item.collected {
                let item_size = item.size();
                let item_rect = Rect::new(
                    item.position.x - item_size / 2.0,
                    item.position.y - item_size / 2.0,
                    item_size,
                    item_size,
                );
                graphics::rectangle(
                    ctx,
                    graphics::DrawParam::default().dest(item_rect.point()),
                    &item_rect,
                    item.color(),
                )?;
            }
        }

        // 绘制分数和时间
        let time_left = GAME_DURATION - (Instant::now() - self.start_time);
        let time_left_seconds = time_left.as_secs();

        let score_text = Text::new(TextFragment::new(format!("Score: {}", self.score))
            .color(Color::WHITE)
            .font_size(24));
        graphics::draw(
            ctx,
            &score_text,
            graphics::DrawParam::default().dest(na::Point2::new(10.0, 10.0)),
        )?;

        let time_text = Text::new(TextFragment::new(format!("Time: {}s", time_left_seconds))
            .color(Color::WHITE)
            .font_size(24));
        graphics::draw(
            ctx,
            &time_text,
            graphics::DrawParam::default().dest(na::Point2::new(SCREEN_WIDTH - 120.0, 10.0)),
        )?;

        // 如果游戏结束，绘制游戏结束界面
        if self.game_over {
            let game_over_text = Text::new(TextFragment::new("Game Over!")
                .color(Color::RED)
                .font_size(48));
            let game_over_rect = game_over_text.dimensions(ctx)?;
            graphics::draw(
                ctx,
                &game_over_text,
                graphics::DrawParam::default().dest(na::Point2::new(
                    SCREEN_WIDTH / 2.0 - game_over_rect.w / 2.0,
                    SCREEN_HEIGHT / 2.0 - 50.0,
                )),
            )?;

            let final_score_text = Text::new(TextFragment::new(format!("Final Score: {}", self.score))
                .color(Color::WHITE)
                .font_size(32));
            let final_score_rect = final_score_text.dimensions(ctx)?;
            graphics::draw(
                ctx,
                &final_score_text,
                graphics::DrawParam::default().dest(na::Point2::new(
                    SCREEN_WIDTH / 2.0 - final_score_rect.w / 2.0,
                    SCREEN_HEIGHT / 2.0 + 10.0,
                )),
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context, dt: f32) -> GameResult {
        self.update(dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, graphics: &mut graphics::GraphicsContext) -> GameResult {
        self.draw(ctx, graphics)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) -> GameResult {
        if self.game_over {
            return Ok(());
        }

        match keycode {
            KeyCode::Left => {
                self.miner.move_left();
            }
            KeyCode::Right => {
                self.miner.move_right();
            }
            KeyCode::Space => {
                // 计算钩子发射角度（基于鼠标位置）
                let mouse_pos = _ctx.mouse.position();
                let angle = (mouse_pos.y - self.miner.position.y).atan2(mouse_pos.x - self.miner.position.x);
                self.hook.throw(angle);
            }
            _ => (),
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("gold_miner", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("黄金矿工"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;
    let mut state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
