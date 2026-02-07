pub fn run() {
    // 구조체 = 다른 언어의 class와 비슷
    let mut player = Player::new("용사", 100);
    println!("  {}", player.status());

    player.take_damage(30);
    println!("  피격 후: {}", player.status());

    player.heal(15);
    println!("  회복 후: {}", player.status());

    // 트레이트 = 인터페이스와 비슷
    let sword = Sword { damage: 25 };
    let staff = Staff { magic_power: 40 };

    print_weapon_info(&sword);
    print_weapon_info(&staff);
}

struct Player {
    name: String,
    hp: i32,
    max_hp: i32,
}

impl Player {
    fn new(name: &str, max_hp: i32) -> Self {
        Player {
            name: name.to_string(),
            hp: max_hp,
            max_hp,
        }
    }

    fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    fn status(&self) -> String {
        format!("{}: HP {}/{}", self.name, self.hp, self.max_hp)
    }
}

// 트레이트 = 공통 행동 정의
trait Weapon {
    fn name(&self) -> &str;
    fn attack_power(&self) -> i32;
    fn description(&self) -> String {
        // 기본 구현 제공 가능
        format!("{} (공격력: {})", self.name(), self.attack_power())
    }
}

struct Sword {
    damage: i32,
}

impl Weapon for Sword {
    fn name(&self) -> &str { "강철 검" }
    fn attack_power(&self) -> i32 { self.damage }
}

struct Staff {
    magic_power: i32,
}

impl Weapon for Staff {
    fn name(&self) -> &str { "마법 지팡이" }
    fn attack_power(&self) -> i32 { self.magic_power }
    fn description(&self) -> String {
        format!("{} (마력: {})", self.name(), self.magic_power)
    }
}

fn print_weapon_info(weapon: &dyn Weapon) {
    println!("  무기: {}", weapon.description());
}
