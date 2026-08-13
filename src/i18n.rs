
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Zh,
    En,
}

impl Lang {
    pub fn toggle(self) -> Self {
        match self {
            Lang::Zh => Lang::En,
            Lang::En => Lang::Zh,
        }
    }
}

pub struct Texts {
    pub title: &'static str,
    pub continue_game: &'static str,
    pub new_game: &'static str,
    pub quit: &'static str,
    pub press_any_key: &'static str,

    // status
    pub day: &'static str,
    pub cash: &'static str,
    pub bank: &'static str,
    pub debt: &'static str,
    pub health: &'static str,
    pub fame: &'static str,
    pub inventory: &'static str,

    // market
    pub market: &'static str,
    pub my_goods: &'static str,
    pub goods_name: &'static str,
    pub price: &'static str,
    pub quantity: &'static str,
    pub avg_cost: &'static str,
    pub buy_quantity: &'static str,
    pub sell_quantity: &'static str,
    pub total: &'static str,
    pub not_enough_cash: &'static str,
    pub not_enough_space: &'static str,

    pub status: &'static str,
    pub facilities: &'static str,
    pub depart: &'static str,
    pub station_hint: &'static str,

    // travel
    pub subway_map: &'static str,
    pub surface_map: &'static str,
    pub already_here: &'static str,

    // locations - subway
    pub loc_jianguomen: &'static str,
    pub loc_beijingzhan: &'static str,
    pub loc_xizhimen: &'static str,
    pub loc_chongwenmen: &'static str,
    pub loc_dongzhimen: &'static str,
    pub loc_fuxingmen: &'static str,
    pub loc_jishuitan: &'static str,
    pub loc_changchunjie: &'static str,
    pub loc_gongzhufen: &'static str,
    pub loc_pingguoyuan: &'static str,

    // locations - surface
    pub loc_yonganli: &'static str,
    pub loc_fangzhuang: &'static str,
    pub loc_haidian: &'static str,
    pub loc_yongdingmen: &'static str,
    pub loc_sanyuanxiqiao: &'static str,
    pub loc_fuyoujie: &'static str,
    pub loc_yayuncun: &'static str,
    pub loc_yuquanying: &'static str,
    pub loc_cuiweiroad: &'static str,
    pub loc_bajiao: &'static str,

    // goods
    pub good_cigarettes: &'static str,
    pub good_cars: &'static str,
    pub good_vcds: &'static str,
    pub good_fake_liquor: &'static str,
    pub good_banned_book: &'static str,
    pub good_toys: &'static str,
    pub good_phones: &'static str,
    pub good_cosmetics: &'static str,

    // facilities
    pub bank_name: &'static str,
    pub hospital_name: &'static str,
    pub post_office: &'static str,
    pub house_agency: &'static str,
    pub internet_cafe: &'static str,
    pub deposit: &'static str,
    pub withdraw: &'static str,
    pub deposit_amount: &'static str,
    pub withdraw_amount: &'static str,
    pub repay_amount: &'static str,

    // hospital
    pub hospital_greeting: &'static str,
    pub hospital_heal_points: &'static str,
    pub hospital_no_money: &'static str,
    pub hospital_healthy: &'static str,
    pub hospital_cost_per_point: &'static str,

    // house
    pub house_too_large: &'static str,
    pub house_no_money: &'static str,
    pub house_expanded: &'static str,
    pub house_min_cash: &'static str,
    pub house_rent_hint: &'static str,

    // cafe
    pub cafe_too_many: &'static str,
    pub cafe_no_money: &'static str,
    pub cafe_reward: &'static str,
    pub cafe_visits: &'static str,
    pub cafe_need_cash: &'static str,
    pub cafe_enter_hint: &'static str,

    // debt
    pub debt_none: &'static str,
    pub debt_not_enough: &'static str,
    pub repay_hint: &'static str,

    // game over
    pub game_over: &'static str,
    pub final_score: &'static str,
    pub going_home: &'static str,
    pub sell_remaining: &'static str,
    pub last_day_warning: &'static str,
    pub death: &'static str,
    pub debt_punishment: &'static str,
    pub play_again: &'static str,
    pub no_rank: &'static str,
    pub broke: &'static str,
    pub rank_label: &'static str,

    // high scores
    pub high_scores: &'static str,
    pub no_scores: &'static str,
    pub name_col: &'static str,
    pub score_col: &'static str,
    pub enter_name: &'static str,
    pub name_hint: &'static str,
    pub default_name: &'static str,

    // events
    pub health_critical: &'static str,
    pub cash_zero: &'static str,
    pub bag_too_small: &'static str,
    pub fame_msg_book: &'static str,
    pub fame_msg_liquor: &'static str,
    pub hacker_decrease: &'static str,
    pub hacker_increase: &'static str,

    // settings
    pub settings: &'static str,
    pub hacker_events: &'static str,
    pub on: &'static str,
    pub off: &'static str,

    // confirm
    pub confirm_new_game: &'static str,

    // popup titles
    pub news: &'static str,
    pub diary: &'static str,

    // hints
    pub hint_menu: &'static str,
    pub hint_market: &'static str,
    pub hint_qty: &'static str,
    pub hint_amount: &'static str,
    pub hint_heal: &'static str,
    pub hint_confirm: &'static str,
    pub hint_back: &'static str,
    pub hint_settings: &'static str,
    pub language_label: &'static str,
}

pub static ZH: Texts = Texts {
    title: "北京浮生记",
    continue_game: "继续游戏",
    new_game: "新游戏",
    quit: "退出",
    press_any_key: "按任意键继续...",

    day: "天",
    cash: "现金",
    bank: "存款",
    debt: "债务",
    health: "健康",
    fame: "声望",
    inventory: "背包",

    market: "黑市",
    my_goods: "我的货物",
    goods_name: "商品",
    price: "价格",
    quantity: "数量",
    avg_cost: "均价",
    buy_quantity: "买入",
    sell_quantity: "卖出",
    total: "总计",
    not_enough_cash: "现金不足!",
    not_enough_space: "背包已满!",

    status: "状 态",
    facilities: "设 施",
    depart: "前往",
    station_hint: "Enter:前往   G:换图",

    subway_map: "北京地铁站",
    surface_map: "北京地面站",
    already_here: "你已经在这里了!",

    loc_jianguomen: "建国门",
    loc_beijingzhan: "北京站",
    loc_xizhimen: "西直门",
    loc_chongwenmen: "崇文门",
    loc_dongzhimen: "东直门",
    loc_fuxingmen: "复兴门",
    loc_jishuitan: "积水潭",
    loc_changchunjie: "长椿街",
    loc_gongzhufen: "公主坟",
    loc_pingguoyuan: "苹果园",

    loc_yonganli: "永安里",
    loc_fangzhuang: "方庄",
    loc_haidian: "海淀大街",
    loc_yongdingmen: "永定门",
    loc_sanyuanxiqiao: "三元西桥",
    loc_fuyoujie: "府右街",
    loc_yayuncun: "亚运村",
    loc_yuquanying: "玉泉营",
    loc_cuiweiroad: "翠微路",
    loc_bajiao: "八角西路",

    good_cigarettes: "进口香烟",
    good_cars: "走私汽车",
    good_vcds: "盗版VCD、游戏",
    good_fake_liquor: "假白酒(剧毒!)",
    good_banned_book: "《上海小宝贝》(禁书)",
    good_toys: "进口玩具",
    good_phones: "水货手机",
    good_cosmetics: "伪劣化妆品",

    bank_name: "银行",
    hospital_name: "医院",
    post_office: "邮局",
    house_agency: "房屋中介",
    internet_cafe: "网吧",
    deposit: "存款",
    withdraw: "取款",
    deposit_amount: "存入金额",
    withdraw_amount: "取出金额",
    repay_amount: "还款金额",

    hospital_greeting: "大夫高兴地拍着手：\"您需要治疗吗?\"",
    hospital_heal_points: "治疗点数",
    hospital_no_money: "医生说：\"钱不够哎! 拒绝治疗。\"",
    hospital_healthy: "小护士笑咪咪地望着俺：\"大哥! 神经科这边挂号.\"",
    hospital_cost_per_point: "每点3500元",

    house_too_large: "中介说，您的房子比局长的还大! 还租房?",
    house_no_money: "中介说，您没有三万现金就想租房? 一边凉快去!",
    house_expanded: "我的房子可以放更多物品了! 可是，好象中介公司骗了我一些钱...",
    house_min_cash: "最低现金要求",
    house_rent_hint: "J:租房(容量+10)   K:返回",

    cafe_too_many: "村长放出话来：你别总是在网吧里鬼混，快去做正经买卖!",
    cafe_no_money: "进网吧至少身上要带15元，呵呵，取钱再来。",
    cafe_reward: "感谢电信改革，可以免费上网! 还挣了美国网络广告费",
    cafe_visits: "已上网次数",
    cafe_need_cash: "需带现金≥15元(不扣除)",
    cafe_enter_hint: "J:进网吧   K:返回",

    debt_none: "你没有债务!",
    debt_not_enough: "村长老婆狂吞\"雪中丐\"补钙片，冷笑道：\"你还得起吗?\"",
    repay_hint: "J:还债   K:返回",

    game_over: "游戏结束",
    final_score: "最终资产",
    going_home: "俺已经在北京40天了，该回去结婚去了。",
    sell_remaining: "系统替我卖了剩余货物。",
    last_day_warning: "俺明天回家乡，快把全部货物卖掉。",
    death: "俺倒在街头,身边日记本上写着：\"北京，我将再来!\"",
    debt_punishment: "俺欠钱太多，村长叫一群老乡揍了俺一顿!",
    play_again: "再玩一把?  J:是   K:返回主菜单",
    no_rank: "没能进入富人前10名，下次努力哦!",
    broke: "在北京没挣着钱，被遣送回家。",
    rank_label: "排名",

    high_scores: "富人榜",
    no_scores: "暂无记录",
    name_col: "姓名",
    score_col: "资产",
    enter_name: "恭喜! 您进入了富人榜，请输入姓名",
    name_hint: "输入姓名后按 Enter 确认",
    default_name: "无名氏",

    health_critical: "俺的健康..健康危机..快去医..",
    cash_zero: "俺不好办了。",
    bag_too_small: "可惜! 俺租的房子太小，只能放",
    fame_msg_book: "买卖《上海小宝贝》(禁书),污染社会,俺的名声变坏了啊!",
    fame_msg_liquor: "买卖假白酒(剧毒!),危害社会，俺的名声下降了.",
    hacker_decrease: "黑客入侵银行网络，疯狂修改数据库，我的存款减少了",
    hacker_increase: "黑客入侵银行网络，疯狂修改数据库，我的存款增加了",

    settings: "设置",
    hacker_events: "黑客事件",
    on: "开",
    off: "关",

    confirm_new_game: "您正在玩一个游戏，要放弃它并开始新的吗?",

    news: "新闻",
    diary: "日记",

    hint_menu: "上下:移动   Enter:选择   Esc:退出",
    hint_market: "WASD/方向键:移动   Enter/J:确认   Q:买   E:卖   B:银行   H:医院   P:邮局   R:中介   N:网吧   G:换图   Esc:菜单",
    hint_qty: "左右:±1   上下:±10   Enter:确认   Esc:取消",
    hint_amount: "左右:±100   上下:±1000   Enter:确认   Esc:取消",
    hint_heal: "左右:±1   上下:±5   Enter:确认   Esc:取消",
    hint_confirm: "Enter:确认   Esc:取消",
    hint_back: "Esc:返回",
    hint_settings: "上下:移动   Enter:切换   Esc:返回",
    language_label: "语言: 中文",
};

pub static EN: Texts = Texts {
    title: "Beijing Drifter",
    continue_game: "Continue",
    new_game: "New Game",
    quit: "Quit",
    press_any_key: "Press any key to continue...",

    day: "Day",
    cash: "Cash",
    bank: "Savings",
    debt: "Debt",
    health: "Health",
    fame: "Fame",
    inventory: "Bag",

    market: "Black Market",
    my_goods: "My Goods",
    goods_name: "Goods",
    price: "Price",
    quantity: "Qty",
    avg_cost: "Avg",
    buy_quantity: "Buy",
    sell_quantity: "Sell",
    total: "Total",
    not_enough_cash: "Not enough cash!",
    not_enough_space: "Bag is full!",

    status: "Status",
    facilities: "Facilities",
    depart: "Travel",
    station_hint: "Enter:Go   G:Map",

    subway_map: "Beijing Subway",
    surface_map: "Beijing Streets",
    already_here: "You are already here!",

    loc_jianguomen: "Jianguomen",
    loc_beijingzhan: "Beijing Station",
    loc_xizhimen: "Xizhimen",
    loc_chongwenmen: "Chongwenmen",
    loc_dongzhimen: "Dongzhimen",
    loc_fuxingmen: "Fuxingmen",
    loc_jishuitan: "Jishuitan",
    loc_changchunjie: "Changchunjie",
    loc_gongzhufen: "Gongzhufen",
    loc_pingguoyuan: "Pingguoyuan",

    loc_yonganli: "Yong'anli",
    loc_fangzhuang: "Fangzhuang",
    loc_haidian: "Haidian Street",
    loc_yongdingmen: "Yongdingmen",
    loc_sanyuanxiqiao: "Sanyuan Bridge",
    loc_fuyoujie: "Fuyou Street",
    loc_yayuncun: "Asian Games Village",
    loc_yuquanying: "Yuquanying",
    loc_cuiweiroad: "Cuiwei Road",
    loc_bajiao: "Bajiao West Road",

    good_cigarettes: "Smuggled Cigarettes",
    good_cars: "Smuggled Cars",
    good_vcds: "Pirated VCDs",
    good_fake_liquor: "Fake Liquor (toxic!)",
    good_banned_book: "\"Shanghai Baby\" (banned)",
    good_toys: "Imported Toys",
    good_phones: "Grey Market Phones",
    good_cosmetics: "Fake Cosmetics",

    bank_name: "Bank",
    hospital_name: "Hospital",
    post_office: "Post Office",
    house_agency: "House Agency",
    internet_cafe: "Internet Cafe",
    deposit: "Deposit",
    withdraw: "Withdraw",
    deposit_amount: "Deposit amount",
    withdraw_amount: "Withdraw amount",
    repay_amount: "Repay amount",

    hospital_greeting: "The doctor rubs his hands: \"Need treatment?\"",
    hospital_heal_points: "Points to heal",
    hospital_no_money: "Doctor: \"Not enough money! Treatment refused.\"",
    hospital_healthy: "Nurse smiles: \"Sir, psychiatry is this way.\"",
    hospital_cost_per_point: "3500 per point",

    house_too_large: "Agent: Your place is bigger than the mayor's! Still renting?",
    house_no_money: "Agent: No 30,000 cash, no deal! Buzz off!",
    house_expanded: "My place can hold more goods now! But the agency ripped me off...",
    house_min_cash: "Minimum cash",
    house_rent_hint: "J:Rent (+10 capacity)   K:Back",

    cafe_too_many: "Village chief: Stop wasting time at the cafe, go make money!",
    cafe_no_money: "Need at least 15 yuan to enter the cafe.",
    cafe_reward: "Free internet! Earned ad revenue: ",
    cafe_visits: "Visits",
    cafe_need_cash: "Requires cash >= 15 (not deducted)",
    cafe_enter_hint: "J:Enter   K:Back",

    debt_none: "You have no debt!",
    debt_not_enough: "Chief's wife sneers: \"You think you can pay that back?\"",
    repay_hint: "J:Repay   K:Back",

    game_over: "Game Over",
    final_score: "Final Score",
    going_home: "40 days in Beijing. Time to go home and get married.",
    sell_remaining: "System auto-sold remaining goods.",
    last_day_warning: "Going home tomorrow. Sell all goods now!",
    death: "Collapsed on the street. Diary reads: \"Beijing, I'll be back!\"",
    debt_punishment: "Debt too high! The village chief sent thugs to beat you up!",
    play_again: "Play again?  J:Yes   K:Main menu",
    no_rank: "Didn't make the top 10. Try harder next time!",
    broke: "Made no money in Beijing. Sent back home.",
    rank_label: "Rank",

    high_scores: "Rich List",
    no_scores: "No records yet",
    name_col: "Name",
    score_col: "Score",
    enter_name: "Congratulations! You made the rich list. Enter your name",
    name_hint: "Type a name, then press Enter",
    default_name: "Anonymous",

    health_critical: "Health critical... need hospital...",
    cash_zero: "I'm in trouble now.",
    bag_too_small: "My place is too small, it can only hold ",
    fame_msg_book: "Dealing banned books pollutes society. My reputation suffers!",
    fame_msg_liquor: "Dealing toxic fake liquor harms society. My reputation drops.",
    hacker_decrease: "Hackers breached the bank! Savings decreased by ",
    hacker_increase: "Hackers breached the bank! Savings increased by ",

    settings: "Settings",
    hacker_events: "Hacker Events",
    on: "ON",
    off: "OFF",

    confirm_new_game: "A game is in progress. Abandon it and start a new one?",

    news: "News",
    diary: "Diary",

    hint_menu: "Up/Down:Move   Enter:Select   Esc:Quit",
    hint_market: "WASD/Arrows:Move   Enter/J:OK   Q:Buy   E:Sell   B:Bank   H:Hosp   P:Post   R:Rent   N:Cafe   G:Map   Esc:Menu",
    hint_qty: "Left/Right:+-1   Up/Down:+-10   Enter:OK   Esc:Cancel",
    hint_amount: "Left/Right:+-100   Up/Down:+-1000   Enter:OK   Esc:Cancel",
    hint_heal: "Left/Right:+-1   Up/Down:+-5   Enter:OK   Esc:Cancel",
    hint_confirm: "Enter:Confirm   Esc:Cancel",
    hint_back: "Esc:Back",
    hint_settings: "Up/Down:Move   Enter:Toggle   Esc:Back",
    language_label: "Lang: English",
};

pub fn texts(lang: Lang) -> &'static Texts {
    match lang {
        Lang::Zh => &ZH,
        Lang::En => &EN,
    }
}
