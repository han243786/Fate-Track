"""BaZi (Eight Characters) Analysis Model
This module implements a simplified BaZi analysis engine inspired by the
traditional Chinese astrological system described in *Sanming Tonghui* and
related works.  It is designed as a demonstrative example rather than a
fully fledged replacement for professional astrological software.  The goal
is to show how one might structure rules, compute elemental strengths and
deliver a narrative report.

The model works with four pillars (year, month, day and hour), each
represented by a tuple of (heavenly stem, earthly branch).  It calculates
basic five‑element counts, determines whether the day master (日主) is strong
or weak, assesses the distribution of the Ten Gods (十神) and builds a
simple narrative.  The output is a dictionary containing structured
information that can be rendered into a human‑readable report by a front‑end
application.

While simplified, the code lays out the key functions you would need for a
more sophisticated engine: rule definitions, scoring, report assembly and
serialization to a binary format.

Note: For brevity the model assumes the user provides the four pillars as
Chinese characters (e.g. "甲子", "乙丑") rather than computing them from a
Gregorian date.  A real implementation should include calendrical
conversion routines or integrate a library such as `china-calendar`.
"""

from __future__ import annotations

import pickle
from dataclasses import dataclass, field
from typing import List, Tuple, Dict, Any, Optional

############################################################
# Helpers and domain definitions
############################################################

# The ten heavenly stems and twelve earthly branches are the building blocks
# of the BaZi system.  We define them here along with a mapping to the
# five elements (五行) for convenience.

# Heavenly stems (天干)
HEAVENLY_STEMS = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"]
# Earthly branches (地支)
EARTHLY_BRANCHES = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"
]

# Mapping of each stem/branch to its dominant element.  In practice
# combinations and hidden stems complicate the picture, but this table is a
# reasonable starting point for educational purposes.
ELEMENT_MAP = {
    # Stems
    "甲": "木", "乙": "木",
    "丙": "火", "丁": "火",
    "戊": "土", "己": "土",
    "庚": "金", "辛": "金",
    "壬": "水", "癸": "水",
    # Branches
    "子": "水", "丑": "土",
    "寅": "木", "卯": "木",
    "辰": "土", "巳": "火",
    "午": "火", "未": "土",
    "申": "金", "酉": "金",
    "戌": "土", "亥": "水",
}

# Ten Gods (十神) mapping relative to the day stem
#
# In classical BaZi theory there are ten relational archetypes describing
# how another heavenly stem interacts with the day stem.  They are split
# into five pairs based on whether the interacting stem shares yin/yang
# polarity with the day stem.  For example, a stem of the same element
# and polarity as the day stem is 比肩 (peer) whereas one of the same element
# but opposite polarity is 劫财 (rob wealth).  The other four
# archetypes—食神/伤官 (output), 正印/偏印 (resource), 正财/偏财 (wealth) and
# 正官/七杀 (authority)—are derived by comparing the generating and
# controlling cycles and the yin/yang polarity.

# Yin/Yang polarity of each heavenly stem
STEM_YIN_YANG: Dict[str, str] = {
    "甲": "yang", "乙": "yin",
    "丙": "yang", "丁": "yin",
    "戊": "yang", "己": "yin",
    "庚": "yang", "辛": "yin",
    "壬": "yang", "癸": "yin",
}

# Hidden stems contained within each earthly branch.  These represent the
# secondary heavenly stem influences that reside within a branch.  Including
# them in the analysis allows for a richer Ten God distribution and more
# nuanced day master strength calculation.
HIDDEN_STEMS_MAP: Dict[str, List[str]] = {
    "子": ["癸"],
    "丑": ["己", "辛", "癸"],
    "寅": ["甲", "丙", "戊"],
    "卯": ["乙"],
    "辰": ["乙", "戊", "癸"],
    "巳": ["丙", "戊", "庚"],
    "午": ["丁", "己"],
    "未": ["己", "丁", "乙"],
    "申": ["庚", "壬", "戊"],
    "酉": ["辛"],
    "戌": ["辛", "丁", "戊"],
    "亥": ["壬", "甲"],
}

# Month stage mapping for the five elements.  According to classical texts
# such as Sanming Tonghui, each element goes through a cycle of
# flourishing (旺), assisting (相), resting (休), imprisoning (囚) and death
# (死) over the twelve Earthly Branches (months).  This table encodes
# that seasonal influence, used to weight the day master strength.
MONTH_STAGE_MAP: Dict[str, Dict[str, str]] = {
    "木": {
        "寅": "相", "卯": "旺", "辰": "余", "巳": "囚", "午": "死",
        "未": "囚", "申": "囚", "酉": "死", "戌": "休", "亥": "休",
        "子": "休", "丑": "囚",
    },
    "火": {
        "寅": "休", "卯": "休", "辰": "囚", "巳": "相", "午": "旺",
        "未": "余", "申": "囚", "酉": "囚", "戌": "死", "亥": "囚",
        "子": "死", "丑": "囚",
    },
    "土": {
        "寅": "死", "卯": "死", "辰": "旺", "巳": "相", "午": "余",
        "未": "相", "申": "囚", "酉": "休", "戌": "旺", "亥": "死",
        "子": "囚", "丑": "相",
    },
    "金": {
        "寅": "囚", "卯": "死", "辰": "休", "巳": "休", "午": "囚",
        "未": "囚", "申": "相", "酉": "旺", "戌": "余", "亥": "休",
        "子": "休", "丑": "余",
    },
    "水": {
        "寅": "囚", "卯": "囚", "辰": "休", "巳": "囚", "午": "死",
        "未": "囚", "申": "休", "酉": "死", "戌": "死", "亥": "旺",
        "子": "旺", "丑": "相",
    },
}

# Assign a numerical weight to each stage.  These values can be tuned to
# influence how strongly seasonality impacts day master strength.  '旺' is
# most supportive, '相' is slightly supportive, '休' is neutral, '囚'
# diminishes and '死' weakens dramatically.
MONTH_STAGE_WEIGHT: Dict[str, float] = {
    "旺": 1.2,
    "相": 1.0,
    "休": 0.8,
    "囚": 0.5,
    "死": 0.3,
    "余": 0.8,  # "余气" treated similar to 休 (some residual support)
}

# Full Ten God map including all ten archetypes.  We'll populate this
# dynamically below using a refined rule set that accounts for yin/yang
# polarity.
FULL_TEN_GODS_MAP: Dict[Tuple[str, str], str] = {}

def build_full_ten_gods_map() -> None:
    """Populate the FULL_TEN_GODS_MAP with detailed Ten God relationships.

    This function derives the Ten Gods classification by comparing the
    elements and yin/yang polarity of the day stem and another heavenly
    stem.  The classification differentiates between 正印/偏印, 正财/偏财,
    正官/七杀, 食神/伤官, 比肩/劫财.  Stems that have no clear relation
    return "其他".
    """
    # Generation and control cycles for the five elements
    generating = {"木": "火", "火": "土", "土": "金", "金": "水", "水": "木"}
    controlling = {"木": "土", "土": "水", "水": "火", "火": "金", "金": "木"}
    for day_stem in HEAVENLY_STEMS:
        day_elem = ELEMENT_MAP[day_stem]
        day_pol = STEM_YIN_YANG[day_stem]
        for other in HEAVENLY_STEMS:
            other_elem = ELEMENT_MAP[other]
            other_pol = STEM_YIN_YANG[other]
            if other == day_stem:
                continue
            # Same element: 比肩 or 劫财 depending on polarity
            if other_elem == day_elem:
                if day_pol == other_pol:
                    role = "比肩"
                else:
                    role = "劫财"
            # Other generates day (resource): 正印/偏印
            elif generating[other_elem] == day_elem:
                if day_pol == other_pol:
                    role = "正印"
                else:
                    role = "偏印"
            # Day generates other (output): 食神/伤官
            elif generating[day_elem] == other_elem:
                if day_pol == other_pol:
                    role = "食神"
                else:
                    role = "伤官"
            # Other controls day (authority): 正官/七杀
            elif controlling[other_elem] == day_elem:
                if day_pol == other_pol:
                    role = "正官"
                else:
                    role = "七杀"
            # Day controls other (wealth): 正财/偏财
            elif controlling[day_elem] == other_elem:
                if day_pol == other_pol:
                    role = "正财"
                else:
                    role = "偏财"
            else:
                role = "其他"
            FULL_TEN_GODS_MAP[(day_stem, other)] = role

# Initialize full Ten God map on module import
build_full_ten_gods_map()


@dataclass
class BaZiReport:
    """Structured report holding intermediate results and narrative.

    Attributes
    ----------
    pillars : List[Tuple[str, str]]
        The four pillars as (stem, branch) pairs for year, month, day and hour.
    five_element_count : Dict[str, int]
        Counts of each of the five elements across the eight characters.
    day_master_strength : str
        Qualitative assessment: "偏旺", "偏弱" or "中和".
    ten_god_distribution : Dict[str, int]
        Counts of each Ten God role relative to the day stem.
    narrative : str
        A human‑readable summary of the analysis.
    """

    pillars: List[Tuple[str, str]]
    five_element_count: Dict[str, int]
    day_master_strength: str
    ten_god_distribution: Dict[str, int]
    narrative: str


class BaZiModel:
    """Simplified BaZi analysis model.

    The model accepts four pillars provided by the user and returns a
    structured BaZiReport.  It can be serialized/deserialized with pickle.
    """

    def __init__(self) -> None:
        # Placeholder for future parameterization
        self.name = "SimplifiedBaZiModel"

    def analyze(
        self,
        pillars: List[Tuple[str, str]],
        gender: Optional[str] = None,
        num_major_fortunes: int = 6,
    ) -> BaZiReport:
        """Analyze a set of four pillars.

        Parameters
        ----------
        pillars : List[Tuple[str, str]]
            A list of four (stem, branch) tuples in order: year, month,
            day, hour.

        Parameters
        ----------
        pillars : List[Tuple[str, str]]
            A list of four (stem, branch) tuples in order: year, month,
            day, hour.
        gender : Optional[str], default None
            Gender of the person being analysed ("男" for male, "女" for female).
            If provided, it will be used when interpreting情感 (relationship) and
            大运 (major fortune) directions.  If not provided, a gender-neutral
            interpretation will be used.
        num_major_fortunes : int, default 6
            Number of major fortune (大运) periods to generate. Each period
            corresponds to one step along the 60-year cycle after the month pillar.

        Returns
        -------
        BaZiReport
            An object containing counts, strength assessment and narrative.
        """
        if len(pillars) != 4:
            raise ValueError("Exactly four pillars must be provided (year, month, day, hour)")

        # Compute five element counts for the eight characters (stems and branches)
        five_counts: Dict[str, int] = {elem: 0 for elem in ["金", "木", "水", "火", "土"]}
        for stem, branch in pillars:
            five_counts[ELEMENT_MAP[stem]] += 1
            five_counts[ELEMENT_MAP[branch]] += 1

        # Determine the day master element and stem
        day_stem = pillars[2][0]
        day_elem = ELEMENT_MAP[day_stem]
        day_pol = STEM_YIN_YANG[day_stem]

        # Improved day master strength calculation
        # We assign weights to each of the eight characters based on how they
        # interact with the day master: same element and generating elements
        # add to strength, while elements that control or are produced by
        # the day master reduce strength.  Hidden stems are also considered.
        generating = {"木": "火", "火": "土", "土": "金", "金": "水", "水": "木"}
        controlling = {"木": "土", "土": "水", "水": "火", "火": "金", "金": "木"}
        # Base strength score
        score = 0.0
        # Gather all stems in the chart, including hidden stems
        all_stems: List[str] = []
        for idx, (stem, branch) in enumerate(pillars):
            # Include the explicit stem
            all_stems.append(stem)
            # Include hidden stems of the branch
            hidden = HIDDEN_STEMS_MAP.get(branch, [])
            all_stems.extend(hidden)
        # Count contributions
        for s in all_stems:
            if s == day_stem:
                continue
            elem = ELEMENT_MAP[s]
            # Same element: strong positive
            if elem == day_elem:
                score += 1.0
            # Generates day: positive (resource)
            elif generating[elem] == day_elem:
                score += 0.8
            # Day generates other: negative (output drains energy)
            elif generating[day_elem] == elem:
                score -= 0.6
            # Controls day: negative (authority dominates)
            elif controlling[elem] == day_elem:
                score -= 1.0
            # Day controls other: slight negative (wealth drains)
            elif controlling[day_elem] == elem:
                score -= 0.6
            # Otherwise no contribution
        # Apply seasonal weight based on month branch
        month_branch = pillars[1][1]
        stage = MONTH_STAGE_MAP.get(day_elem, {}).get(month_branch, "休")
        weight = MONTH_STAGE_WEIGHT.get(stage, 0.8)
        weighted_score = score * weight
        # Determine qualitative strength from weighted score
        if weighted_score >= 3.0:
            strength = "偏旺"
        elif weighted_score <= 0.0:
            strength = "偏弱"
        else:
            strength = "中和"

        # Ten God distribution relative to day stem using full mapping
        roles = [
            "比肩", "劫财", "食神", "伤官", "偏财", "正财", "正官", "七杀", "偏印", "正印", "其他"
        ]
        dist: Dict[str, int] = {r: 0 for r in roles}
        for idx, (stem, branch) in enumerate(pillars):
            # Skip the day pillar itself for the main stem; but include branch hidden stems
            if idx == 2:
                # include hidden stems of the day branch, since they still interact
                for hidden_stem in HIDDEN_STEMS_MAP.get(branch, []):
                    role = FULL_TEN_GODS_MAP.get((day_stem, hidden_stem), "其他")
                    dist[role] += 1
                continue
            # Main stem
            role = FULL_TEN_GODS_MAP.get((day_stem, stem), "其他")
            dist[role] += 1
            # Hidden stems within branch
            for hidden_stem in HIDDEN_STEMS_MAP.get(branch, []):
                role = FULL_TEN_GODS_MAP.get((day_stem, hidden_stem), "其他")
                dist[role] += 1

        # Build narrative
        narrative_lines: List[str] = []
        narrative_lines.append(
            f"日主五行属于{day_elem}（{day_stem}），阴阳属性为{day_pol}，整体表现为{strength}。"
        )
        # Element summary
        narrative_lines.append(
            f"五行分布：木{five_counts['木']} 火{five_counts['火']} 土{five_counts['土']} 金{five_counts['金']} 水{five_counts['水']}。"
        )
        max_elem = max(five_counts, key=five_counts.get)
        min_elem = min(five_counts, key=five_counts.get)
        narrative_lines.append(
            f"此命局中{max_elem}较旺，{min_elem}相对较弱，应以补{min_elem}、调理{max_elem}为宜。"
        )
        # Ten Gods interpretation with explanation
        tg_parts = []
        for r in roles:
            if dist.get(r, 0) > 0:
                tg_parts.append(f"{r}{dist[r]}")
        narrative_lines.append("十神分布：" + ", ".join(tg_parts))
        narrative_lines.append(
            "比肩/劫财代表竞争与自我意识，印星代表学习与贵人，食神/伤官象征表达与创意，财星指代资源与财富，官杀寓意压力与责任。根据这些结构可进一步细化职业、性格、财务等方面的倾向。"
        )
        narrative_lines.append(
            "总体而言，建议命主充分发挥所长，同时关注弱项的补救，平衡五行能量，并结合大运流年进一步分析。该分析仅供学习与参考，不可替代专业咨询。"
        )
        # Interpret personality, career, finance and emotions based on Ten God distribution
        personality_desc = self._interpret_personality(dist)
        career_desc = self._interpret_career(dist)
        finance_desc = self._interpret_finance(dist)
        emotion_desc = self._interpret_emotion(dist, gender)

        narrative_lines.append("")
        narrative_lines.append("【性格倾向】" + personality_desc)
        narrative_lines.append("【职业方向】" + career_desc)
        narrative_lines.append("【财务状况】" + finance_desc)
        narrative_lines.append("【情感关系】" + emotion_desc)

        # Major fortune analysis
        fortunes = self._calculate_major_fortune(pillars, day_stem, num_periods=num_major_fortunes, gender=gender)
        if fortunes:
            narrative_lines.append("")
            narrative_lines.append("【大运流年概述】")
            for idx, f in enumerate(fortunes, 1):
                narrative_lines.append(
                    f"第{idx}大运：{f['stem']}{f['branch']}（{f['ten_god']}） - {f['description']}"
                )

        narrative_lines.append(
            "总体而言，建议命主充分发挥所长，同时关注弱项的补救，平衡五行能量，并结合大运流年进一步分析。该分析仅供学习与参考，不可替代专业咨询。"
        )

        report = BaZiReport(
            pillars=pillars,
            five_element_count=five_counts,
            day_master_strength=strength,
            ten_god_distribution=dist,
            narrative="\n".join(narrative_lines),
        )
        return report

    # ------------------------------------------------------------------
    # Interpretation helpers
    # ------------------------------------------------------------------
    def _aggregate_pairs(self, dist: Dict[str, int]) -> Dict[str, int]:
        """Aggregate counts into five paired groups.

        Parameters
        ----------
        dist : Dict[str, int]
            Distribution of the 10 Ten Gods.

        Returns
        -------
        Dict[str, int]
            Dictionary with keys "比肩", "印星", "食伤", "财星", "官杀" representing the
            total counts for each paired group.
        """
        pairs = {
            "比肩": dist.get("比肩", 0) + dist.get("劫财", 0),
            "印星": dist.get("正印", 0) + dist.get("偏印", 0),
            "食伤": dist.get("食神", 0) + dist.get("伤官", 0),
            "财星": dist.get("正财", 0) + dist.get("偏财", 0),
            "官杀": dist.get("正官", 0) + dist.get("七杀", 0),
        }
        return pairs

    def _interpret_personality(self, dist: Dict[str, int]) -> str:
        """Generate a personality description based on Ten God distribution.

        Parameters
        ----------
        dist : Dict[str, int]
            Ten God distribution.

        Returns
        -------
        str
            A sentence describing personality tendencies.
        """
        pairs = self._aggregate_pairs(dist)
        # Identify dominant categories
        max_val = max(pairs.values()) if pairs else 0
        dominant = [k for k, v in pairs.items() if v == max_val and v > 0]
        desc_parts: List[str] = []
        # Map to personality traits
        for cat in dominant:
            if cat == "印星":
                desc_parts.append("内敛沉稳，重视学习与传统，思考深入")
            elif cat == "比肩":
                desc_parts.append("自我意识强，独立且不易妥协")
            elif cat == "食伤":
                desc_parts.append("表达力强，热爱创作与享乐，思维活跃")
            elif cat == "财星":
                desc_parts.append("务实现实，重视物质与效率，善于资源配置")
            elif cat == "官杀":
                desc_parts.append("讲究规矩，责任心强，注重社会评价")
        if not desc_parts:
            return "性格平衡，无明显偏向。"
        return "，".join(desc_parts) + "。"

    def _interpret_career(self, dist: Dict[str, int]) -> str:
        """Suggest career directions based on Ten God distribution.

        Returns
        -------
        str
            A sentence suggesting suitable careers.
        """
        pairs = self._aggregate_pairs(dist)
        # Determine primary influence
        max_val = max(pairs.values()) if pairs else 0
        dominant = [k for k, v in pairs.items() if v == max_val and v > 0]
        suggestions: List[str] = []
        for cat in dominant:
            if cat == "印星":
                suggestions.append("教学、研究、法律、心理咨询等需要专业知识与深度思考的行业")
            elif cat == "比肩":
                suggestions.append("创业、自媒体、体育、竞技等需要自我驱动和敢于挑战的行业")
            elif cat == "食伤":
                suggestions.append("文化创意、传媒艺术、演艺、餐饮等需要创意表达的行业")
            elif cat == "财星":
                suggestions.append("商业贸易、金融投资、管理咨询、房地产等经营运作领域")
            elif cat == "官杀":
                suggestions.append("政府机关、司法、军警、企业管理等注重制度与责任的行业")
        if not suggestions:
            return "适合多元领域，可根据个人兴趣自由选择。"
        return "，或".join(suggestions) + "。"

    def _interpret_finance(self, dist: Dict[str, int]) -> str:
        """Assess financial tendencies.

        Returns
        -------
        str
            A sentence describing financial behaviour.
        """
        pairs = self._aggregate_pairs(dist)
        wealth = pairs.get("财星", 0)
        output = pairs.get("食伤", 0)
        resource = pairs.get("印星", 0)
        authority = pairs.get("官杀", 0)
        peer = pairs.get("比肩", 0)
        desc_parts: List[str] = []
        if wealth >= max(output, resource, authority, peer):
            desc_parts.append("财星旺，擅于理财和积累，但需防偏财过旺带来的风险")
        if output > wealth and output >= max(resource, authority, peer):
            desc_parts.append("食伤突出，收入多来自创意或口才，但财务波动较大")
        if resource > wealth and resource >= max(output, authority, peer):
            desc_parts.append("印星为主，重视知识与资质，金钱观念较弱")
        if authority > wealth and authority >= max(output, resource, peer):
            desc_parts.append("官杀显著，财务来源稳定，多靠薪资或制度保障")
        if peer > max(wealth, output, resource, authority):
            desc_parts.append("比肩劫财突出，支出较大，需谨慎合作与借贷")
        if not desc_parts:
            return "财务状况平稳，宜量入为出，理性规划。"
        return "；".join(desc_parts) + "。"

    def _interpret_emotion(self, dist: Dict[str, int], gender: Optional[str]) -> str:
        """Provide insights on relationships and emotions.

        Returns
        -------
        str
            A sentence describing relationship tendencies.
        """
        pairs = self._aggregate_pairs(dist)
        # Determine spouse star based on gender
        spouse_star = None
        spouse_desc = []
        if gender:
            g = gender.strip().lower()
            if g in ["男", "male", "m", "boy"]:
                spouse_star = "财星"
            elif g in ["女", "female", "f", "girl"]:
                spouse_star = "官杀"
        # Evaluate spouse star
        if spouse_star:
            count = pairs.get(spouse_star, 0)
            if count >= 4:
                spouse_desc.append("配偶星强，婚恋重视承诺与现实条件")
            elif count == 0:
                spouse_desc.append("配偶星弱，情感道路曲折且容易晚婚")
            else:
                spouse_desc.append("配偶星适中，婚姻质量取决于自我成长和沟通")
        # Additional notes based on other stars
        if pairs.get("食伤", 0) > pairs.get("印星", 0):
            spouse_desc.append("表达力旺盛，但过于锋利易招口舌是非")
        if pairs.get("比肩", 0) + pairs.get("官杀", 0) > pairs.get("财星", 0) + pairs.get("印星", 0):
            spouse_desc.append("性格独立或责任压力大，需注意平衡家庭与事业")
        if pairs.get("财星", 0) > pairs.get("官杀", 0) and spouse_star != "财星":
            spouse_desc.append("财富星旺，对伴侣的物质需求较高")
        if pairs.get("印星", 0) > pairs.get("食伤", 0) and spouse_star != "官杀":
            spouse_desc.append("内心传统，感情中重视安全与信任")
        if not spouse_desc:
            return "情感关系平稳，重视沟通与包容。"
        return "；".join(spouse_desc) + "。"

    # ------------------------------------------------------------------
    # Major fortune (大运) calculation
    # ------------------------------------------------------------------
    def _next_pillar(self, stem: str, branch: str, step: int) -> Tuple[str, str]:
        """Compute the pillar that is `step` positions ahead in the 60‑year cycle.

        The heavenly stems cycle every 10 positions and earthly branches every 12.
        This function simply increments both indices by the same step, which
        approximates the movement along the sexagenary cycle.

        Parameters
        ----------
        stem : str
            The starting heavenly stem.
        branch : str
            The starting earthly branch.
        step : int
            Number of positions to move forward along the cycle.

        Returns
        -------
        Tuple[str, str]
            The resulting (stem, branch) pair.
        """
        stem_idx = HEAVENLY_STEMS.index(stem)
        branch_idx = EARTHLY_BRANCHES.index(branch)
        new_stem = HEAVENLY_STEMS[(stem_idx + step) % len(HEAVENLY_STEMS)]
        new_branch = EARTHLY_BRANCHES[(branch_idx + step) % len(EARTHLY_BRANCHES)]
        return (new_stem, new_branch)

    def _calculate_major_fortune(
        self,
        pillars: List[Tuple[str, str]],
        day_stem: str,
        num_periods: int = 6,
        gender: Optional[str] = None,
    ) -> List[Dict[str, Any]]:
        """Generate a sequence of major fortune periods.

        A simplified approach that moves along the sexagenary cycle starting
        from the month pillar.  Each period corresponds to the next combination
        of stem and branch.  Direction of progression may vary by gender and
        yin/yang in traditional theory; here we simply move forward.

        Parameters
        ----------
        pillars : List[Tuple[str, str]]
            Four pillars (year, month, day, hour).
        day_stem : str
            Heavenly stem of the day pillar for Ten God comparison.
        num_periods : int, optional
            Number of fortune periods to generate, by default 6.
        gender : Optional[str], optional
            Gender of the individual, by default None.

        Returns
        -------
        List[Dict[str, Any]]
            A list of dictionaries describing each major fortune period.
        """
        if num_periods <= 0:
            return []
        month_stem, month_branch = pillars[1]
        fortunes: List[Dict[str, Any]] = []
        for i in range(1, num_periods + 1):
            f_stem, f_branch = self._next_pillar(month_stem, month_branch, i)
            # Determine Ten God relation of fortune stem to day stem
            ten_god = FULL_TEN_GODS_MAP.get((day_stem, f_stem), "其他")
            # Simple description based on Ten God
            description = self._fortune_description(ten_god)
            fortunes.append({
                "index": i,
                "stem": f_stem,
                "branch": f_branch,
                "ten_god": ten_god,
                "description": description,
            })
        return fortunes

    def _fortune_description(self, ten_god: str) -> str:
        """Return a generic description for a major fortune based on Ten God.

        Parameters
        ----------
        ten_god : str
            Ten God type of the fortune's heavenly stem relative to the day
            stem.

        Returns
        -------
        str
            A short description.
        """
        if ten_god in ("正印", "偏印"):
            return "学习提升、自我反省与内在成长的阶段"
        if ten_god in ("比肩", "劫财"):
            return "竞争与自我挑战增多，适宜自主发展"
        if ten_god in ("食神", "伤官"):
            return "才华展现、创意迸发的时期，也需注意言行"
        if ten_god in ("正财", "偏财"):
            return "财运活跃、资源交换频繁，宜抓住机会"
        if ten_god in ("正官", "七杀"):
            return "责任与压力上升，可谋求职位晋升或角色转变"
        return "平和顺遂，无明显起伏"

    def save_binary(self, filepath: str) -> None:
        """Serialize the model to a binary file using pickle.

        Parameters
        ----------
        filepath : str
            Path to the file where the binary representation will be saved.
        """
        with open(filepath, "wb") as f:
            pickle.dump(self, f)

    @staticmethod
    def load_binary(filepath: str) -> "BaZiModel":
        """Deserialize the model from a binary file.

        Parameters
        ----------
        filepath : str
            Path to the binary file produced by `save_binary`.

        Returns
        -------
        BaZiModel
            The reconstructed model instance.
        """
        with open(filepath, "rb") as f:
            obj = pickle.load(f)
        if not isinstance(obj, BaZiModel):
            raise TypeError("Loaded object is not a BaZiModel")
        return obj
