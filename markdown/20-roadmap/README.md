# 开发里程碑目录

从这里开始读：

## 当前状态

- `v1.0.0-preview` 已发布。
- M0-M28 已关闭。
- V1 preview 运行时能力矩阵：10 supported、7 restricted；post-preview 当前运行态：10 supported、14 restricted、0 planned。
- 用户已取消 post-preview 功能边界锁，并授权四个专项切面纳入规划边界：情感、金钱、家庭、事业。
- RPT-004 深度研究报告已完成治理承接；DG-011 已设计关闭；LOOP-093 已采纳 M29 implementation preflight 决策；LOOP-094 已完成 M29/M30。
- 当前游标已完成 M31-M33、LOOP-096 四专题白话化硬化，并完成 LOOP-097 专项工作台摘要/完整报告页分层。LOOP-098 已采纳大运/流年重型优化方案并铺设 M34-M40。
- LOOP-099 已通过 ADR 0022 关闭 DG-012，并完成 M35 内部 `domain::timeline` foundation。LOOP-100 已完成 M36 主盘大运解释，`luck-reading` 通过 `/api/charts/report?reading_year=YYYY` 进入 restricted。LOOP-101 已完成 M37 年度引动解释，`annual-trigger-reading` 通过 `/api/charts/report?year=YYYY` 进入 restricted。LOOP-102 已完成 M38 四专项大运流年叠加，`topic-timeline-reading` 通过 `/api/charts/topic-report?topic=...&year=YYYY` 进入 restricted。LOOP-103 已完成 M39 timeline report UI：报告页时间导航、显式年份控件、可展开证据和工作台短摘要。LOOP-104 已完成 M40 timeline quality gate closeout：golden samples、forbidden/no-score/no-overclaim、bounded-output、frontend source boundary 和治理闭环均已登记。LOOP-105 已完成前端质量修正：重新起盘清空旧专项栏，可见内部英文标识转中文，无能力变化。LOOP-106 已完成 timeline 词典文案质量门禁：拦截僵硬标签、年份病句、内部工程口径和后端/前端说明。LOOP-107 已完成大规模 timeline 词典优化：28 个组合式条目覆盖十神、五行、五行流向、地支关系、藏干、格局、用神，并强化词典原文和生成文案密度门禁。LOOP-108 已完成报告级强约束：主盘报告与四专项完整报告最终 API 正文统一通过可见文案禁用词、自然中文、显式年份、无内部工程口径、无 public score 门禁，不改变能力状态。
- LOOP-109 已完成 M41 情感专项报告叙事打磨：只改 `relationship-report` 单一切面，正文固定六块，开头一次提醒，年度情感引动合并和证据折叠保留；不改变 API route、DTO 顶层结构、capability 状态或 raw `luck-cycles`。
- LOOP-110 已完成 M42 情感专项真实输出再打磨：先审读真实生成样本，再压下“标记、筛选、提取、当前关系”等机器口吻；仍只处理 `relationship-report` 单一切面，不改变 capability 状态。
- LOOP-111 已完成 M43 剩余报告真实输出再打磨：主盘、情感、金钱、家庭、事业五份 assembled report 真实样本均为 0 禁用命中、0 ASCII word；主盘、金钱、家庭、事业正文已清除内部英文、等号证据、机器口吻和可见后端变量；不改变 capability 状态。
- LOOP-112 已完成 M44 情感专项真实输出二次门禁：解决固定开头复读，把 `被冲牵动` 改为 `被"冲"牵动` 这类术语引号化表达，并把同类回归纳入五份真实样本文案扫描；不改变 capability 状态。
- LOOP-113 已完成 M45 五份真实报告系统口吻清理：主盘不再展示算法/系统/评分口吻，四专项不再展示计数表式摘要，timeline 大运标签改为 `第一运·丙子` 这类可读形式；不改变 capability 状态。
- LOOP-114 已完成 M46 五份真实报告清单口吻叙事化：主盘年度引动、四专项时间叠加和情感/专题计数不再回到锚点清单、十条/五条统计或“出现几处”台账；不改变 capability 状态。
- LOOP-117 已完成 M49 年度/大运流年叙事基线：主盘年度引动与三专项大运流年从清单证据改为连贯读盘顺序，并继续不改变 capability 状态。
- LOOP-118 已完成 M50 三专项解盘凝聚：金钱、家庭、事业从说明书口吻改为 `总断`、专题入口和关键词结论，并把 `日常读法`、`日常看`、`这些牵动提醒您` 等旧词纳入门禁；不改变 capability 状态。
- LOOP-119 已完成 M51 主盘报告语气凝聚：主盘章节从教学说明改为读盘正文，十神摘要从 `比肩一处` 这类计数台账改为结构信号，并继续不改变 capability 状态。
- LOOP-120 已完成 M52 报告收束连续性：主盘旧列表/排序口吻继续压实，金钱、家庭、事业的大运流年章节置于结论前并以专题结论收尾；不改变 capability 状态。
- LOOP-121 已完成 M53 报告密度与专题化：主盘五行解释合并成分组读法，金钱、家庭、事业时间段改成贴题建议，并把 `偏弱表示这类倾向`、`哪里需要放慢`、`哪里需要承接`、`读2026年这一层` 纳入门禁；不改变 capability 状态。
- LOOP-128 已完成 M60 三专项大运流年读者口吻打磨：金钱、家庭、事业 `本专题的大运流年` 从 `从「...」专项来看` / `十神与五行这一层` / `藏干...合到一起时` 这类层级说明改成直接读 2026 年专题节奏；不改变 capability 状态。
- LOOP-129 已完成 M61 年龄语境报告打磨：2025/2026 早年样本不再按成人恋爱、收入、投资、岗位或职业结果呈现，改为情绪回应、资源感、稳定照护、学习任务、规则感和表达训练；不改变 capability 状态。
- Timeline 解释层 M34-M40 已收口；所有 timeline 能力仍为 restricted，上限不得绕过显式年份、不公开 `score_internal`、不宣称完整流年/流月系统，任何 supported promotion 必须另开里程碑和 ADR。后续词典扩展必须通过 LOOP-107 的自然中文与组合式词典质量门禁，后续报告正文变更必须通过 LOOP-108 的最终可见文本门禁。
- `relationship-report`、`wealth-report`、`family-report`、`career-report` 当前均为 restricted，不得宣称为 supported 或确定性现实事件建议；情感报告可保留低风险关系节奏建议。

1. `00-roadmap-index.md`
2. `90-decision-gates.md`
3. `91-anti-regression-and-governance-lock.md`
4. `95-recursive-development-protocol.md`
5. `96-recursive-cursor.md`
6. `97-loop-closeout-log.md`
7. `98-recursive-loop-runbook.md`
8. `100-recursive-scale-and-goal-readiness.md`
9. 如需示例，读 `99-milestone-01-preflight-dry-run.md`
10. 当前要执行的里程碑文件
11. 如关闭 M1，读 `11-milestone-01-closeout-readiness.md`
12. 如复核 M1 closeout，读 `12-milestone-01-closeout.md`
13. 如复核 M2，读 `13-milestone-02-preflight.md` 和 `14-milestone-02-closeout.md`
14. 如复核 M3，读 `15-milestone-03-preflight.md` 和 `16-milestone-03-closeout.md`
15. 如复核 M4，读 `17-milestone-04-preflight.md` 和 `18-milestone-04-closeout.md`
16. 如复核 M8，读 `25-milestone-08-preflight.md`、`26-milestone-08-closeout.md` 和 `docs/release/v1-release-candidate.md`
17. 如进入 M9，读 `27-milestone-09-preflight.md`、`docs/decisions/0015-m9-astronomy-parallel-strategy.md` 和 `data/generated/astronomy/README.md`
18. `93-capability-promotion-ledger.md`
19. `94-closeout-evidence-template.md`
20. post-preview 新切面推进前，读 `89-post-preview-documentation-freeze.md`
21. M29-M33 专项报告里程碑文件

## 执行原则

- 先完成里程碑治理，再落实代码。
- 任何 supported 晋级必须有证据。
- 任何回退必须记录原因、范围和替代保护。
- 任何新 public surface 必须同步模块树和工程树。
- 递归游标处于 `design_only` 时，不推进业务代码、API、UI 实现或 capability 晋级。
- 每轮递归采用 largest stable invariant：一个不变量内尽量多落代码，出现第二个不变量立即拆轮。

## Post-Preview Reference

- M23 astronomy-engine promotion: `83-milestone-23-astronomy-engine-promotion.md`
- M24 chart report: `84-milestone-24-chart-report.md`
- M25 frontend visual upgrade: `85-milestone-25-frontend-visual-upgrade.md`
- M26 report portal redesign: `86-milestone-26-report-portal-redesign.md`
- M27 colloquial report content: `87-milestone-27-colloquial-report-content.md`
- M28 desktop shell: `88-milestone-28-desktop-shell.md`
- Post-preview freeze and four-slice intake: `89-post-preview-documentation-freeze.md`
- RPT-004 topic-report research intake: `../reserch/04-topic-report-engine-governance-intake.md`
- M29 topic report foundation: `101-milestone-29-topic-report-foundation.md`
- M30 relationship topic report: `102-milestone-30-relationship-topic-report.md`
- M31 wealth topic report: `103-milestone-31-wealth-topic-report.md`
- M32 family topic report: `104-milestone-32-family-topic-report.md`
- M33 career topic report: `105-milestone-33-career-topic-report.md`
- M34 timeline reading governance: `106-milestone-34-timeline-reading-governance.md`
- M35 timeline lexicon and rule engine: `107-milestone-35-timeline-lexicon-rule-engine.md`
- M34 closeout readiness: `113-milestone-34-closeout-readiness.md`
- M35 closeout: `114-milestone-35-closeout.md`
- M36 primary chart luck reading: `108-milestone-36-primary-chart-luck-reading.md`
- M36 closeout: `115-milestone-36-closeout.md`
- M37 annual trigger reading: `109-milestone-37-annual-trigger-reading.md`
- M37 closeout: `116-milestone-37-closeout.md`
- M38 topic timeline overlay: `110-milestone-38-topic-timeline-overlay.md`
- M38 closeout: `117-milestone-38-closeout.md`
- M39 timeline report UI: `111-milestone-39-timeline-report-ui.md`
- M39 closeout: `118-milestone-39-closeout.md`
- M40 timeline quality gate and closeout: `112-milestone-40-timeline-quality-gate-closeout.md`
- M40 closeout: `119-milestone-40-closeout.md`
- M41 relationship report narrative polish: `120-milestone-41-relationship-report-narrative-polish.md`
- M41 closeout: `121-milestone-41-closeout.md`
- M42 relationship report human copy gate: `122-milestone-42-relationship-report-human-copy-gate.md`
- M42 closeout: `123-milestone-42-closeout.md`
- M43 remaining report human copy gate: `124-milestone-43-remaining-report-human-copy-gate.md`
- M43 closeout: `125-milestone-43-closeout.md`
- M44 relationship copy second-pass gate: `126-milestone-44-relationship-copy-second-pass.md`
- M44 closeout: `127-milestone-44-closeout.md`
- M45 report system-tone cleanup gate: `128-milestone-45-report-system-tone-cleanup.md`
- M45 closeout: `129-milestone-45-closeout.md`
- M46 report narrative list cleanup gate: `130-milestone-46-report-narrative-list-cleanup.md`
- M46 closeout: `131-milestone-46-closeout.md`
- M47 relationship golden sample baseline: `132-milestone-47-relationship-golden-sample-baseline.md`
- M47 closeout: `133-milestone-47-closeout.md`
- M48 topic count-field narrative baseline: `134-milestone-48-topic-count-field-narrative-baseline.md`
- M48 closeout: `135-milestone-48-closeout.md`
- M49 annual timeline narrative baseline: `136-milestone-49-annual-timeline-narrative-baseline.md`
- M49 closeout: `137-milestone-49-closeout.md`
- M50 topic report advice cohesion: `138-milestone-50-topic-report-advice-cohesion.md`
- M50 closeout: `139-milestone-50-closeout.md`
- M51 main report tone cohesion: `140-milestone-51-main-report-tone-cohesion.md`
- M51 closeout: `141-milestone-51-closeout.md`
- M52 report closeout continuity: `142-milestone-52-report-closeout-continuity.md`
- M52 closeout: `143-milestone-52-closeout.md`
- M53 report density and topic specificity: `144-milestone-53-report-density-topic-specificity.md`
- M53 closeout: `145-milestone-53-closeout.md`
- M54 timeline detail narrative warmth: `146-milestone-54-timeline-detail-narrative-warmth.md`
- M54 closeout: `147-milestone-54-closeout.md`
- M55 current luck consistency and annual decompression: `148-milestone-55-current-luck-consistency-and-annual-decompression.md`
- M55 closeout: `149-milestone-55-closeout.md`
- M56 report conclusion de-duplication and topic personality: `150-milestone-56-report-conclusion-de-duplication.md`
- M56 closeout: `151-milestone-56-closeout.md`
- M57 timeline prose de-staging: `152-milestone-57-timeline-prose-de-staging.md`
- M57 closeout: `153-milestone-57-closeout.md`
- M58 main report long-section condensation: `154-milestone-58-main-report-long-section-condensation.md`
- M58 closeout: `155-milestone-58-closeout.md`
- M59 topic middle-chapter personality polish: `156-milestone-59-topic-middle-chapter-personality-polish.md`
- M59 closeout: `157-milestone-59-closeout.md`
- M60 topic timeline reader-facing polish: `158-milestone-60-topic-timeline-reader-facing-polish.md`
- M60 closeout: `159-milestone-60-closeout.md`
- M61 age context report polish: `160-milestone-61-age-context-report-polish.md`
- M61 closeout: `161-milestone-61-closeout.md`
## M5 Reference

- M5 preflight: `19-milestone-05-preflight.md`
- M5 closeout: `20-milestone-05-closeout.md`
- M5 restricted capabilities: `case-management`, `settings`
- M6 preflight: `21-milestone-06-preflight.md`
- M6 closeout: `22-milestone-06-closeout.md`
- M6 restricted capability: `share-preview`
- M7 preflight: `23-milestone-07-preflight.md`
- M7 closeout: `24-milestone-07-closeout.md`
- M7 restricted capabilities: `frontend-chart-workspace`, `frontend-share-preview`
- M8 preflight: `25-milestone-08-preflight.md`
- M8 closeout: `26-milestone-08-closeout.md`
- M8 supported governance capability: `release-candidate`
- M9 preflight: `27-milestone-09-preflight.md`
- M9 ADRs: `docs/decisions/0015-m9-astronomy-parallel-strategy.md`, `docs/decisions/0016-m9-astronomy-source-stack.md`
- M9 source availability: `28-milestone-09-source-availability.md`
- M9 manifest draft: `29-milestone-09-manifest-draft.md`
- M9 generation plan: `30-milestone-09-generation-plan.md`
- M9 generator dry-run: `31-milestone-09-generator-dry-run.md`
- M9 comparison/golden/replay plan: `32-milestone-09-comparison-golden-replay-plan.md`
- M9 comparison dry-run: `33-milestone-09-comparison-dry-run.md`
- M9 golden-case dry-run: `34-milestone-09-golden-dry-run.md`
- M9 replay-policy dry-run: `35-milestone-09-replay-policy-dry-run.md`
- M9 pre-closeout audit: `36-milestone-09-pre-closeout-audit.md`
- M9 generated-data implementation plan: `37-milestone-09-generated-data-implementation-plan.md`
- M9 generator contract: `38-milestone-09-generator-contract.md`
- M9 source adapter contract: `39-milestone-09-source-adapter-contract.md`
- M9 artifact writer dry-run: `40-milestone-09-artifact-writer-dry-run.md`
- M9 comparison runner dry-run: `41-milestone-09-comparison-runner-dry-run.md`
- M9 golden-row readiness: `42-milestone-09-golden-row-readiness.md`
- M9 replay-test readiness: `43-milestone-09-replay-test-readiness.md`
- M9 preflight closeout: `44-milestone-09-preflight-closeout.md`
- M10 generated astronomy implementation: `45-milestone-10-generated-astronomy-implementation.md`
- M10 generator implementation entry: `46-milestone-10-generator-entry.md`
- M10 source snapshot manifest boundary: `47-milestone-10-source-snapshot-boundary.md`
- M10 source snapshot manifest metadata: `48-milestone-10-source-snapshot-manifest.md`
- M10 source payload materialization policy: `49-milestone-10-source-payload-policy.md`
- M10 source payload schemas: `50-milestone-10-source-payload-schemas.md`
- M10 source capture procedure: `51-milestone-10-source-capture-procedure.md`
- M10 first source payload decision: `52-milestone-10-first-source-payload-decision.md`
- M10 selected source payload preflight: `53-milestone-10-selected-source-payload-preflight.md`
- M10 selected source payload materialization: `54-milestone-10-selected-source-payload-materialization.md`
- M10 remaining source payload strategy: `55-milestone-10-remaining-source-payload-strategy.md`
- M10 selected IAU SOFA payload preflight: `56-milestone-10-selected-iau-sofa-payload-preflight.md`
- M10 selected IAU SOFA payload materialization: `57-milestone-10-selected-iau-sofa-payload-materialization.md`
- M10 post-IAU remaining source payload strategy: `58-milestone-10-post-iau-remaining-source-payload-strategy.md`
- M10 selected JPL Horizons payload preflight: `59-milestone-10-selected-jpl-horizons-payload-preflight.md`
- M10 selected JPL Horizons payload materialization: `60-milestone-10-selected-jpl-horizons-payload-materialization.md`
- M10 selected GB/T payload preflight: `61-milestone-10-selected-gb-t-payload-preflight.md`
- M10 selected GB/T payload materialization: `62-milestone-10-selected-gb-t-payload-materialization.md`
- M9 target capability: `astronomy-engine`
