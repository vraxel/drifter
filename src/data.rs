use crate::i18n::{Lang, texts};

pub const GOOD_COUNT: usize = 8;
pub const LOCATION_COUNT: usize = 10;

/// Cursor movement and the rendered rows read the same column count, so the
/// UI never has to report its width back to the input handler. English names
/// are long enough that three per row would not fit.
pub fn location_grid_cols(lang: Lang) -> usize {
    match lang {
        Lang::Zh => 3,
        Lang::En => 2,
    }
}

pub const INITIAL_CASH: i64 = 2000;
pub const INITIAL_DEBT: i64 = 5000;
pub const INITIAL_BANK: i64 = 0;
pub const INITIAL_HEALTH: i32 = 100;
pub const INITIAL_FAME: i32 = 100;
pub const INITIAL_CAPACITY: i32 = 100;
pub const MAX_CAPACITY: i32 = 140;
pub const TOTAL_DAYS: i32 = 40;
pub const MAX_CAFE_VISITS: i32 = 3;
pub const CAFE_COST: i64 = 15;
pub const HOSPITAL_COST_PER_POINT: i64 = 3500;
pub const HOUSE_MIN_CASH: i64 = 30000;
pub const DEBT_PUNISHMENT_THRESHOLD: i64 = 100000;
pub const DEBT_PUNISHMENT_DAMAGE: i32 = 30;
pub const DEBT_INTEREST_RATE: f64 = 0.10;
pub const BANK_INTEREST_RATE: f64 = 0.01;
pub const AUTO_HOSPITAL_HEALTH_THRESHOLD: i32 = 85;
pub const AUTO_HOSPITAL_MIN_DAYS_LEFT: i32 = 3;
pub const AUTO_HOSPITAL_HEAL: i32 = 10;

pub struct GoodDef {
    pub base_price: i64,
    pub price_range: i64,
    pub fame_penalty: i32,
}

pub static GOODS: [GoodDef; GOOD_COUNT] = [
    GoodDef { base_price: 100,   price_range: 350,   fame_penalty: 0 },   // 0: cigarettes
    GoodDef { base_price: 15000, price_range: 15000, fame_penalty: 0 },   // 1: cars
    GoodDef { base_price: 5,     price_range: 50,    fame_penalty: 0 },   // 2: vcds
    GoodDef { base_price: 1000,  price_range: 2500,  fame_penalty: 10 },  // 3: fake liquor
    GoodDef { base_price: 5000,  price_range: 9000,  fame_penalty: 7 },   // 4: banned book
    GoodDef { base_price: 250,   price_range: 600,   fame_penalty: 0 },   // 5: toys
    GoodDef { base_price: 750,   price_range: 750,   fame_penalty: 0 },   // 6: phones
    GoodDef { base_price: 65,    price_range: 180,   fame_penalty: 0 },   // 7: cosmetics
];

pub fn good_name(index: usize, lang: Lang) -> &'static str {
    let t = texts(lang);
    match index {
        0 => t.good_cigarettes,
        1 => t.good_cars,
        2 => t.good_vcds,
        3 => t.good_fake_liquor,
        4 => t.good_banned_book,
        5 => t.good_toys,
        6 => t.good_phones,
        7 => t.good_cosmetics,
        _ => "",
    }
}

pub fn location_names(subway: bool, lang: Lang) -> [&'static str; LOCATION_COUNT] {
    let t = texts(lang);
    if subway {
        [
            t.loc_jianguomen, t.loc_beijingzhan, t.loc_xizhimen,
            t.loc_chongwenmen, t.loc_dongzhimen, t.loc_fuxingmen,
            t.loc_jishuitan, t.loc_changchunjie, t.loc_gongzhufen,
            t.loc_pingguoyuan,
        ]
    } else {
        [
            t.loc_yonganli, t.loc_fangzhuang, t.loc_haidian,
            t.loc_yongdingmen, t.loc_sanyuanxiqiao, t.loc_fuyoujie,
            t.loc_yayuncun, t.loc_yuquanying, t.loc_cuiweiroad,
            t.loc_bajiao,
        ]
    }
}

pub struct MarketEvent {
    pub freq: u32,
    pub msg_zh: &'static str,
    pub msg_en: &'static str,
    pub good_index: usize,
    pub multiply: i64,
    pub divide: i64,
    pub add_items: i32,
    pub extra_debt: i64,
}

pub static MARKET_EVENTS: [MarketEvent; 18] = [
    MarketEvent { freq: 170, msg_zh: "专家提议提高大学生\"动手素质\"，进口玩具颇受欢迎!", msg_en: "Experts promote \"hands-on skills\" for students. Imported toys are popular!", good_index: 5, multiply: 2, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 139, msg_zh: "有人自豪地说：生病不用打针吃药，喝假白酒（剧毒）就可以!", msg_en: "Someone claims: No need for medicine, just drink fake liquor!", good_index: 3, multiply: 3, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 100, msg_zh: "医院的秘密报告：\"《上海小宝贝》功效甚过伟哥\"!", msg_en: "Hospital leak: \"Shanghai Baby works better than Viagra\"!", good_index: 4, multiply: 5, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 41, msg_zh: "文盲说：\"2000年诺贝尔文学奖？呸！不如盗版VCD港台片。\"", msg_en: "\"Nobel Prize? Bah! Pirated VCDs are better!\"", good_index: 2, multiply: 4, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 37, msg_zh: "《北京经济小报》社论：\"走私汽车大力推进汽车消费!\"", msg_en: "Beijing Times: \"Smuggled cars boost auto consumption!\"", good_index: 1, multiply: 3, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 23, msg_zh: "《北京真理报》社论：\"提倡爱美，落到实处\"，伪劣化妆品大受欢迎!", msg_en: "Beijing Truth: \"Beauty first!\" Fake cosmetics in high demand!", good_index: 7, multiply: 4, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 37, msg_zh: "8858.com电子书店也不敢卖《上海小宝贝》，黑市一册可卖天价!", msg_en: "Even online stores dare not sell Shanghai Baby. Black market price is sky-high!", good_index: 4, multiply: 8, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 15, msg_zh: "谢不疯在晚会上说：\"我酷!我使用伪劣化妆品!\"，伪劣化妆品供不应求!", msg_en: "Celebrity endorses fake cosmetics: \"I'm cool! I use them!\" Supply can't keep up!", good_index: 7, multiply: 7, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 40, msg_zh: "北京有人狂饮山西假酒，可以卖出天价!", msg_en: "Beijing people binge-drinking fake liquor. Prices skyrocket!", good_index: 3, multiply: 7, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 29, msg_zh: "北京的大学生们开始找工作，水货手机大受欢迎！!", msg_en: "College grads job-hunting. Grey market phones in demand!", good_index: 6, multiply: 7, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 35, msg_zh: "北京的富人疯狂地购买走私汽车！价格狂升!", msg_en: "Beijing's rich rush to buy smuggled cars! Prices soaring!", good_index: 1, multiply: 8, divide: 0, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 17, msg_zh: "市场上充斥着来自福建的走私香烟!", msg_en: "Market flooded with smuggled cigarettes from Fujian!", good_index: 0, multiply: 0, divide: 8, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 24, msg_zh: "北京的孩子们都忙于上网学习，进口玩具没人愿意买。", msg_en: "Kids busy studying online. Nobody wants imported toys.", good_index: 5, multiply: 0, divide: 5, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 18, msg_zh: "盗版业十分兴旺，\"中国硅谷\"——中关村全是卖盗版VCD的村姑!", msg_en: "Piracy booming! Zhongguancun full of VCD sellers!", good_index: 2, multiply: 0, divide: 8, add_items: 0, extra_debt: 0 },
    MarketEvent { freq: 160, msg_zh: "厦门的老同学资助俺两部走私汽车！发了！！", msg_en: "Old classmate in Xiamen gifted me 2 smuggled cars! Jackpot!", good_index: 1, multiply: 0, divide: 0, add_items: 2, extra_debt: 0 },
    MarketEvent { freq: 45, msg_zh: "工商局扫荡后，俺在黑暗角落里发现了老乡丢失的进口香烟。", msg_en: "Found lost smuggled cigarettes in a dark corner after a raid.", good_index: 0, multiply: 0, divide: 0, add_items: 6, extra_debt: 0 },
    MarketEvent { freq: 35, msg_zh: "俺老乡回家前把一些山西假白酒（剧毒）给俺!", msg_en: "Fellow villager gave me some fake liquor before going home!", good_index: 3, multiply: 0, divide: 0, add_items: 4, extra_debt: 0 },
    MarketEvent { freq: 140, msg_zh: "媒体报道：又有日本出口到中国的产品出事了! 村长硬卖给您一部水货手机，收您2500元。", msg_en: "Japanese products recalled! Village chief forces you to buy a grey phone for 2500.", good_index: 6, multiply: 0, divide: 0, add_items: 1, extra_debt: 2500 },
];

pub struct HealthEvent {
    pub freq: u32,
    pub msg_zh: &'static str,
    pub msg_en: &'static str,
    pub damage: i32,
}

pub static HEALTH_EVENTS: [HealthEvent; 12] = [
    HealthEvent { freq: 117, msg_zh: "大街上两个流氓打了俺!", msg_en: "Two thugs beat you up on the street!", damage: 3 },
    HealthEvent { freq: 157, msg_zh: "俺在过街地道被人打了蒙棍!", msg_en: "Got clubbed in the underpass!", damage: 20 },
    HealthEvent { freq: 21,  msg_zh: "工商局的追俺超过三个胡同。", msg_en: "Commerce bureau chased you through three alleys.", damage: 1 },
    HealthEvent { freq: 100, msg_zh: "北京拥挤的交通让俺心焦!", msg_en: "Beijing's traffic is driving you crazy!", damage: 1 },
    HealthEvent { freq: 35,  msg_zh: "开小巴的打俺一耳光!", msg_en: "Minibus driver slapped you!", damage: 1 },
    HealthEvent { freq: 313, msg_zh: "一群民工打了俺!", msg_en: "A mob of workers beat you up!", damage: 10 },
    HealthEvent { freq: 120, msg_zh: "附近胡同的一个小青年砸俺一砖头!", msg_en: "A punk hit you with a brick!", damage: 5 },
    HealthEvent { freq: 29,  msg_zh: "附近写字楼一个假保安用电棍电击俺!", msg_en: "A fake security guard tased you!", damage: 3 },
    HealthEvent { freq: 43,  msg_zh: "北京臭黑的小河熏着我了!", msg_en: "The stinky river made you sick!", damage: 1 },
    HealthEvent { freq: 45,  msg_zh: "守自行车的王大婶嘲笑俺没北京户口!", msg_en: "Bike-watch auntie mocked your outsider status!", damage: 1 },
    HealthEvent { freq: 48,  msg_zh: "北京高温40度!俺热...", msg_en: "40C heatwave in Beijing! So hot...", damage: 1 },
    HealthEvent { freq: 33,  msg_zh: "申奥添了新风景，北京又来沙尘暴!", msg_en: "Sandstorm hits Beijing again!", damage: 1 },
];

pub struct StealEvent {
    pub freq: u32,
    pub msg_zh: &'static str,
    pub msg_en: &'static str,
    pub loss_percent: i64,
    pub targets_bank: bool,
}

pub static STEAL_EVENTS: [StealEvent; 7] = [
    StealEvent { freq: 60,  msg_zh: "俺怜悯地铁口扮演成乞丐的老太太。", msg_en: "You pitied the old lady pretending to be a beggar.", loss_percent: 10, targets_bank: false },
    StealEvent { freq: 125, msg_zh: "一个汉子在街头拦住俺：\"哥们，给点钱用!\"", msg_en: "A man stops you: \"Buddy, spare some cash!\"", loss_percent: 10, targets_bank: false },
    StealEvent { freq: 100, msg_zh: "一个大个子碰了俺一下，说：\"别挤了!\"", msg_en: "A big guy bumped you: \"Stop pushing!\"", loss_percent: 40, targets_bank: false },
    StealEvent { freq: 65,  msg_zh: "三个带红袖章的老太太揪住俺：\"你是外地人?罚款!\"", msg_en: "Three patrol grannies: \"Outsider? Fine!\"", loss_percent: 20, targets_bank: false },
    StealEvent { freq: 35,  msg_zh: "两个猛男揪住俺：\"交长话附加费、上网费。\"", msg_en: "Two brutes: \"Pay the phone and internet surcharge.\"", loss_percent: 15, targets_bank: true },
    StealEvent { freq: 27,  msg_zh: "副主任说：\"办经商证?晚上不要去我家给我送钱哦。\"", msg_en: "Official: \"Business permit? Don't bring me money tonight.\"", loss_percent: 10, targets_bank: true },
    StealEvent { freq: 40,  msg_zh: "北京空气污染得厉害,俺去氧吧吸氧...", msg_en: "Air pollution so bad, had to visit an oxygen bar...", loss_percent: 5, targets_bank: false },
];

pub struct AutoHospitalLocation {
    pub zh: &'static str,
    pub en: &'static str,
}

pub static AUTO_HOSPITAL_LOCATIONS: [AutoHospitalLocation; 20] = [
    AutoHospitalLocation { zh: "建国门", en: "Jianguomen" },
    AutoHospitalLocation { zh: "北京站", en: "Beijing Station" },
    AutoHospitalLocation { zh: "西直门", en: "Xizhimen" },
    AutoHospitalLocation { zh: "崇文门", en: "Chongwenmen" },
    AutoHospitalLocation { zh: "东直门", en: "Dongzhimen" },
    AutoHospitalLocation { zh: "复兴门", en: "Fuxingmen" },
    AutoHospitalLocation { zh: "积水潭", en: "Jishuitan" },
    AutoHospitalLocation { zh: "长椿街", en: "Changchunjie" },
    AutoHospitalLocation { zh: "公主坟", en: "Gongzhufen" },
    AutoHospitalLocation { zh: "苹果园", en: "Pingguoyuan" },
    AutoHospitalLocation { zh: "永安里", en: "Yong'anli" },
    AutoHospitalLocation { zh: "方庄", en: "Fangzhuang" },
    AutoHospitalLocation { zh: "海淀大街", en: "Haidian Street" },
    AutoHospitalLocation { zh: "永定门", en: "Yongdingmen" },
    AutoHospitalLocation { zh: "三元东桥", en: "Sanyuan Bridge" },
    AutoHospitalLocation { zh: "文津街", en: "Wenjin Street" },
    AutoHospitalLocation { zh: "北辰西路", en: "Beichen West Road" },
    AutoHospitalLocation { zh: "菜户营", en: "Caihuying" },
    AutoHospitalLocation { zh: "翠微路", en: "Cuiwei Road" },
    AutoHospitalLocation { zh: "八角地铁", en: "Bajiao Metro" },
];

pub static COLLAPSE_PLACES_ZH: [&str; 29] = [
    "发廊里", "早点摊上", "报摊上", "烤羊肉摊上", "公共汽车里",
    "人力车上", "女厕所里", "男厕所里", "电话亭里", "三陪女怀里",
    "出租车里", "小巴里", "美容院里", "小商亭里", "小商场门口",
    "民工脚下", "无照游商摊里", "草地上", "电线杆顶端", "小饭馆里",
    "马路边", "人行道上", "街心公园里", "广告牌下", "公共汽车站里",
    "长途汽车站里", "卖盗版游戏的旁边", "网络公司尸体旁边", "行骗的知本家旁边",
];

pub static COLLAPSE_PLACES_EN: [&str; 29] = [
    "a hair salon", "a breakfast stall", "a newsstand", "a kebab stand", "a bus",
    "a rickshaw", "the women's restroom", "the men's restroom", "a phone booth", "an escort's arms",
    "a taxi", "a minibus", "a beauty salon", "a kiosk", "a shop entrance",
    "migrant workers' feet", "an unlicensed vendor's stall", "the grass", "a telephone pole", "a small restaurant",
    "the roadside", "the sidewalk", "a park", "a billboard", "a bus stop",
    "a coach station", "a pirate game seller", "a dot-com corpse", "a con artist",
];
