// M14: Glossary data — structured terminology for stems, branches, elements, and ten gods.
use crate::http::{Response, json};

struct GlossaryEntry {
    id: &'static str,
    term_zh: &'static str,
    term_en: &'static str,
    category: &'static str,
    description_zh: &'static str,
}

pub fn glossary(request: Option<&str>, category: Option<&str>) -> Response {
    let all = glossary_data();
    let filtered: Vec<&GlossaryEntry> = all
        .iter()
        .filter(|e| {
            let term_match = request.map_or(true, |t| {
                e.term_zh.contains(t) || e.term_en.to_lowercase().contains(&t.to_lowercase())
            });
            let cat_match = category.map_or(true, |c| e.category == c);
            term_match && cat_match
        })
        .collect();

    let items: Vec<String> = filtered
        .iter()
        .map(|e| {
            format!(
                "{{\"id\":{},\"term_zh\":{},\"term_en\":{},\"category\":{},\"description_zh\":{}}}",
                json::string(e.id),
                json::string(e.term_zh),
                json::string(e.term_en),
                json::string(e.category),
                json::string(e.description_zh)
            )
        })
        .collect();

    Response::json(format!(
        "{{\"glossary\":[{}],\"count\":{}}}",
        items.join(","),
        filtered.len()
    ))
}

fn glossary_data() -> Vec<GlossaryEntry> {
    vec![
        // Ten stems
        GlossaryEntry {
            id: "stem-jia",
            term_zh: "甲",
            term_en: "Jia",
            category: "stem",
            description_zh: "十天干之首，属阳木，象征大树、领袖、开始。",
        },
        GlossaryEntry {
            id: "stem-yi",
            term_zh: "乙",
            term_en: "Yi",
            category: "stem",
            description_zh: "十天干之二，属阴木，象征花草、柔韧、辅助。",
        },
        GlossaryEntry {
            id: "stem-bing",
            term_zh: "丙",
            term_en: "Bing",
            category: "stem",
            description_zh: "十天干之三，属阳火，象征太阳、热情、光明。",
        },
        GlossaryEntry {
            id: "stem-ding",
            term_zh: "丁",
            term_en: "Ding",
            category: "stem",
            description_zh: "十天干之四，属阴火，象征灯火、细致、温暖。",
        },
        GlossaryEntry {
            id: "stem-wu",
            term_zh: "戊",
            term_en: "Wu",
            category: "stem",
            description_zh: "十天干之五，属阳土，象征城墙、厚重、稳定。",
        },
        GlossaryEntry {
            id: "stem-ji",
            term_zh: "己",
            term_en: "Ji",
            category: "stem",
            description_zh: "十天干之六，属阴土，象征田园、孕育、包容。",
        },
        GlossaryEntry {
            id: "stem-geng",
            term_zh: "庚",
            term_en: "Geng",
            category: "stem",
            description_zh: "十天干之七，属阳金，象征刀剑、决断、变革。",
        },
        GlossaryEntry {
            id: "stem-xin",
            term_zh: "辛",
            term_en: "Xin",
            category: "stem",
            description_zh: "十天干之八，属阴金，象征首饰、精致、修正。",
        },
        GlossaryEntry {
            id: "stem-ren",
            term_zh: "壬",
            term_en: "Ren",
            category: "stem",
            description_zh: "十天干之九，属阳水，象征江河、流动、智慧。",
        },
        GlossaryEntry {
            id: "stem-gui",
            term_zh: "癸",
            term_en: "Gui",
            category: "stem",
            description_zh: "十天干之十，属阴水，象征雨露、渗透、滋养。",
        },
        // Twelve branches
        GlossaryEntry {
            id: "branch-zi",
            term_zh: "子",
            term_en: "Zi",
            category: "branch",
            description_zh: "十二地支之首，属水，生肖鼠，方位北，时辰23-01时。",
        },
        GlossaryEntry {
            id: "branch-chou",
            term_zh: "丑",
            term_en: "Chou",
            category: "branch",
            description_zh: "十二地支之二，属土，生肖牛，时辰01-03时。",
        },
        GlossaryEntry {
            id: "branch-yin",
            term_zh: "寅",
            term_en: "Yin",
            category: "branch",
            description_zh: "十二地支之三，属木，生肖虎，时辰03-05时。",
        },
        GlossaryEntry {
            id: "branch-mao",
            term_zh: "卯",
            term_en: "Mao",
            category: "branch",
            description_zh: "十二地支之四，属木，生肖兔，时辰05-07时。",
        },
        GlossaryEntry {
            id: "branch-chen",
            term_zh: "辰",
            term_en: "Chen",
            category: "branch",
            description_zh: "十二地支之五，属土，生肖龙，时辰07-09时。",
        },
        GlossaryEntry {
            id: "branch-si",
            term_zh: "巳",
            term_en: "Si",
            category: "branch",
            description_zh: "十二地支之六，属火，生肖蛇，时辰09-11时。",
        },
        GlossaryEntry {
            id: "branch-wu",
            term_zh: "午",
            term_en: "Wu",
            category: "branch",
            description_zh: "十二地支之七，属火，生肖马，时辰11-13时。",
        },
        GlossaryEntry {
            id: "branch-wei",
            term_zh: "未",
            term_en: "Wei",
            category: "branch",
            description_zh: "十二地支之八，属土，生肖羊，时辰13-15时。",
        },
        GlossaryEntry {
            id: "branch-shen",
            term_zh: "申",
            term_en: "Shen",
            category: "branch",
            description_zh: "十二地支之九，属金，生肖猴，时辰15-17时。",
        },
        GlossaryEntry {
            id: "branch-you",
            term_zh: "酉",
            term_en: "You",
            category: "branch",
            description_zh: "十二地支之十，属金，生肖鸡，时辰17-19时。",
        },
        GlossaryEntry {
            id: "branch-xu",
            term_zh: "戌",
            term_en: "Xu",
            category: "branch",
            description_zh: "十二地支之十一，属土，生肖狗，时辰19-21时。",
        },
        GlossaryEntry {
            id: "branch-hai",
            term_zh: "亥",
            term_en: "Hai",
            category: "branch",
            description_zh: "十二地支之十二，属水，生肖猪，时辰21-23时。",
        },
        // Five elements
        GlossaryEntry {
            id: "elem-wood",
            term_zh: "木",
            term_en: "Wood",
            category: "element",
            description_zh: "五行之首，方位东，季节春，颜色青，五脏肝。主仁、生长、条达。",
        },
        GlossaryEntry {
            id: "elem-fire",
            term_zh: "火",
            term_en: "Fire",
            category: "element",
            description_zh: "五行之二，方位南，季节夏，颜色赤，五脏心。主礼、温暖、向上。",
        },
        GlossaryEntry {
            id: "elem-earth",
            term_zh: "土",
            term_en: "Earth",
            category: "element",
            description_zh: "五行之三，方位中，季节长夏，颜色黄，五脏脾。主信、承载、化育。",
        },
        GlossaryEntry {
            id: "elem-metal",
            term_zh: "金",
            term_en: "Metal",
            category: "element",
            description_zh: "五行之四，方位西，季节秋，颜色白，五脏肺。主义、收敛、变革。",
        },
        GlossaryEntry {
            id: "elem-water",
            term_zh: "水",
            term_en: "Water",
            category: "element",
            description_zh: "五行之五，方位北，季节冬，颜色黑，五脏肾。主智、润下、藏精。",
        },
        // Ten gods
        GlossaryEntry {
            id: "tg-peer",
            term_zh: "比肩",
            term_en: "Peer",
            category: "ten_god",
            description_zh: "与日主同五行同阴阳。代表兄弟姐妹、同辈、竞争、自我。",
        },
        GlossaryEntry {
            id: "tg-rob",
            term_zh: "劫财",
            term_en: "Rob Wealth",
            category: "ten_god",
            description_zh: "与日主同五行异阴阳。代表朋友、合作者、劫夺、分享。",
        },
        GlossaryEntry {
            id: "tg-eating",
            term_zh: "食神",
            term_en: "Eating God",
            category: "ten_god",
            description_zh: "日主所生同阴阳。代表才华、口福、创造力、自由。",
        },
        GlossaryEntry {
            id: "tg-hurting",
            term_zh: "伤官",
            term_en: "Hurting Officer",
            category: "ten_god",
            description_zh: "日主所生异阴阳。代表聪明、叛逆、艺术、挑战权威。",
        },
        GlossaryEntry {
            id: "tg-dw",
            term_zh: "正财",
            term_en: "Direct Wealth",
            category: "ten_god",
            description_zh: "日主所克异阴阳。代表正当收入、妻子、稳定财源。",
        },
        GlossaryEntry {
            id: "tg-iw",
            term_zh: "偏财",
            term_en: "Indirect Wealth",
            category: "ten_god",
            description_zh: "日主所克同阴阳。代表意外之财、父亲、投资、慷慨。",
        },
        GlossaryEntry {
            id: "tg-do",
            term_zh: "正官",
            term_en: "Direct Officer",
            category: "ten_god",
            description_zh: "克日主异阴阳。代表上司、规则、丈夫、纪律、名誉。",
        },
        GlossaryEntry {
            id: "tg-7k",
            term_zh: "七杀",
            term_en: "Seven Killings",
            category: "ten_god",
            description_zh: "克日主同阴阳。代表压力、挑战、权威、武职、魄力。",
        },
        GlossaryEntry {
            id: "tg-dr",
            term_zh: "正印",
            term_en: "Direct Resource",
            category: "ten_god",
            description_zh: "生日主异阴阳。代表母亲、学识、文凭、慈悲、保护。",
        },
        GlossaryEntry {
            id: "tg-ir",
            term_zh: "偏印",
            term_en: "Indirect Resource",
            category: "ten_god",
            description_zh: "生日主同阴阳。代表继母、特殊技能、孤独、玄学、灵感。",
        },
        // Solar terms
        GlossaryEntry {
            id: "st-xiaohan",
            term_zh: "小寒",
            term_en: "Minor Cold",
            category: "solar_term",
            description_zh: "二十四节气之二十三，太阳黄经285°，约1月5-7日。标志季冬开始。",
        },
        GlossaryEntry {
            id: "st-lichun",
            term_zh: "立春",
            term_en: "Spring Begins",
            category: "solar_term",
            description_zh: "二十四节气之首，太阳黄经315°，约2月3-5日。春季开始，传统岁首。",
        },
        GlossaryEntry {
            id: "st-dongzhi",
            term_zh: "冬至",
            term_en: "Winter Solstice",
            category: "solar_term",
            description_zh: "二十四节气之二十二，太阳黄经270°，约12月21-23日。日照最短，一阳初生。",
        },
    ]
}
