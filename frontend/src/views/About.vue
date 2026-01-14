<template>
  <div class="about-container">
    <n-card :bordered="false" class="main-card" size="huge">
      
      <!-- 1. 头部 Header -->
      <div class="header-section">
        <n-avatar
          :size="100"
          src="/logo.png"
          fallback-src="https://via.placeholder.com/96?text=THO"
          class="logo"
          bordered
        />
        <h1 class="app-title">漫展出摊机</h1>
        <p class="app-subtitle">基于 Tauri 构建的现代化出摊一体工具，旨在改进同人摊主工作流</p>
        
        <n-flex justify="center" size="small" class="tag-row">
          <n-tag :bordered="false" type="info" round size="small">
            <template #icon><n-icon><LogoTux /></n-icon></template> Tauri v2
          </n-tag>
          <n-tag :bordered="false" type="primary" round size="small">
            <template #icon><n-icon><LogoWindows /></n-icon></template> Windows
          </n-tag>
          <n-tag :bordered="false" type="success" round size="small">
            <template #icon><n-icon><LogoAndroid /></n-icon></template> Android
          </n-tag>
          <n-tag :bordered="false" type="warning" round size="small">MIT License</n-tag>
        </n-flex>
      </div>

      <n-divider />
     <section class="section intro-section">
      <n-blockquote>
        “同人展会的魅力在于创作者与同好之间的交流。我不希望摊主们被杂乱的账本、卡顿的网络和繁琐的计算束缚。
        该工具的初衷，是把‘出摊’变得更简单、更优雅，让摊主能更多享受展会的快乐，而非被琐碎事项所束缚。”
      </n-blockquote>
    </section>

      <!-- 2. 项目背景 -->
      <section class="section">
        <div class="section-header">
          <n-icon size="24" color="#18a058"><BookOutline /></n-icon>
          <h2>项目时间线</h2>
        </div>
        <n-timeline size="large">
          <n-timeline-item type="error" title="痛点：混乱的纸质记录" time="2025年8月">
            <span class="text-muted">暑假接手社团主催后，面对混乱库存和纸质账目的痛点，决定开发自动化工具。</span>
          </n-timeline-item>
          <n-timeline-item type="warning" title="尝试：Web 服务版" time="2025年9月-11月">
            <span class="text-muted">开发 Flask+Vue 的原型并部署在社团云服务器，确立了核心工作流。</span>
          </n-timeline-item>
          <n-timeline-item type="warning" title="发布: Nuitka打包版" time="2025年11月">
            <span class="text-muted">尝试本地化打包分发，但受限于 Python 后端体积与平台限制难以分发。</span>
          </n-timeline-item>
          <n-timeline-item type="success" title="现在：Tauri 本地应用" time="2025年12月-至今">
            <span class="text-muted">使用 Rust 重写后端，Tauri v2 构建，实现 Win/Android 双平台高性能体验。</span>
          </n-timeline-item>
        </n-timeline>
      </section>

      <!-- 3. 核心功能 (使用 Grid 组件处理响应式) -->
      <section class="section">
        <div class="section-header">
          <n-icon size="24" color="#2080f0"><HardwareChipOutline /></n-icon>
          <h2>核心功能</h2>
        </div>
        <n-grid x-gap="16" y-gap="16" cols="1 s:1 m:3" responsive="screen">
          <n-grid-item v-for="feature in features" :key="feature.title">
            <n-card class="feature-card" content-style="padding: 16px;" hoverable embedded>
              <n-flex align="center" :wrap="false" class="feature-header">
                <n-icon size="26" :color="feature.color">
                  <component :is="feature.icon" />
                </n-icon>
                <span class="feature-title">{{ feature.title }}</span>
              </n-flex>
              <p class="feature-desc">{{ feature.desc }}</p>
            </n-card>
          </n-grid-item>
        </n-grid>
      </section>

      <!-- 4. 技术栈 -->
      <section class="section">
        <div class="section-header">
          <n-icon size="24" color="#8a2be2"><CodeSlashOutline /></n-icon>
          <h2>技术架构</h2>
        </div>
        <n-flex size="small" class="mb-2">
          <n-tag :bordered="false" type="success">Vue 3 + Naive UI</n-tag>
          <n-tag :bordered="false" type="info">Rust (Axum)</n-tag>
          <n-tag :bordered="false" type="default">SQLite</n-tag>
          <n-tag :bordered="false" type="warning">Tauri v2</n-tag>
        </n-flex>
        <p class="text-muted">
          Naive UI 提供现代化交互，Rust 负责高性能 HTTP 服务与业务逻辑，SQLite 确保数据本地化存储。
        </p>
      </section>

      <!-- 5. 致谢 (使用 Grid 简化布局) -->
      <section class="section">
        <div class="section-header">
          <n-icon size="24" color="#d03050"><HeartOutline /></n-icon>
          <h2>致谢 & 鸣谢</h2>
        </div>
        <n-grid x-gap="12" y-gap="12" cols="1 s:2" responsive="screen">
          <n-grid-item v-for="item in credits" :key="item.title">
            <div class="credit-item">
              <div class="credit-title">{{ item.title }}</div>
              <div class="credit-desc">{{ item.desc }}</div>
            </div>
          </n-grid-item>
        </n-grid>
      </section>

      <n-divider />
      
      <!-- 6. 使用指南与声明 -->
      <section class="section">
        <div class="section-header">
          <n-icon size="24" color="#555"><DocumentTextOutline /></n-icon>
          <h2>指南 & 声明</h2>
        </div>
        
        <n-collapse arrow-placement="right" :default-expanded-names="['free','privacy', 'qa']">
           <n-collapse-item name="free">
            <template #header>
              <n-flex align="center" size="small">
                <n-icon color="#d03050"><GiftOutline /></n-icon>
                <span style="font-weight: 700; color: var(--n-text-color);">永久免费声明</span>
              </n-flex>
            </template>
            <div class="collapse-inner">
              <n-alert type="error" :show-icon="false" class="mb-2">
                <strong>🚫 谨防诈骗</strong>
                <br/>
                本软件为开源项目，<strong>永远免费</strong>。市面上任何出售本软件、源码或“代部署”的行为均为诈骗。
              </n-alert>
              <p class="text-muted">
                如果您是在任何地方付费获取该软件，请立即申请退款并举报。
                您永远可以在 GitHub 或官网免费下载最新版，并直接向开发者反馈问题。
              </p>
            </div>
          </n-collapse-item>
          <n-collapse-item name="privacy">
            <template #header>
              <n-flex align="center" size="small">
                <n-icon color="#18a058"><ShieldCheckmarkOutline /></n-icon>
                <span>数据隐私</span>
              </n-flex>
            </template>
            <div class="collapse-inner">
              <n-alert type="success" :show-icon="false" class="mb-2">
                <strong>本地优先架构</strong><br/>
                你的所有数据（库存、账目、图片）仅存储于设备的 SQLite 数据库中，软件除自动更新检查外不进行任何网络通信。
              </n-alert>
              <p class="text-muted">我们承诺这个项目尽可能允许同好本地原子化部署，无需中心化的服务器，也不会搜集任何隐私数据。</p>
            </div>
          </n-collapse-item>

          <n-collapse-item name="qa">
            <template #header>
               <n-flex align="center" size="small">
                <n-icon color="#2080f0"><WifiOutline /></n-icon>
                <span>常见问题：</span>
               </n-flex>
            </template>
            <div class="collapse-inner">
              <p class="qa-q">❓ 手机扫码后网页无法加载 / 白屏？</p>
              <ul class="qa-list">
                <li><strong>检查网络</strong>：确保主机与扫码设备在同一 Wi-Fi 下(校园网等大型网络环境可能不允许)。</li>
                <li><strong>防火墙</strong>：Windows 请检查是否放行了应用端口。</li>
                <li class="highlight-item">
                  <strong>✨ 推荐方案</strong>：由手机/平板开热点组建局域网，稳定且不耗流量。
                </li>
              </ul>
              <p class="qa-q">❓ 我该如何上手使用？</p>
              <ul class="qa-list">
                <li><strong>配置全局商品</strong>：在“全局商品库”页面根据社团制品信息，添加或修改商品条目。</li>
                <li><strong>创建展会</strong>：在“展会管理”页面创建新的展会，填写好展会信息和收款码图片。</li>
                <li><strong>配置展会商品</strong>：进入展会页面，配置展会贩卖商品的种类、备货量和价格。</li>
                <li><strong>开启和贩卖</strong>：在“展会管理”页面设置展会状态，开始销售，把一个打开了“顾客视图”的设备(最好为平板)摆放在摊位前，即可进行出摊。</li>
                <li><strong>检查和备货</strong>：摊主用一个连接且打开“摊主视图”的设备检查当前展会的订单，并完成订单，进行配货。</li>
                <li><strong>结算</strong>：结束后可以在展会页面查看该展会的销售统计，并导出一个总结性质的Excel文档。</li>
              </ul>
              <p class="qa-q">❓ 我的数据没能同步？</p>
              <ul class="qa-list">
                <li><strong>单一主机</strong>：我们需要的是一台设备运行程序，其它设备扫码局域网连接，这样才能保证数据同步。</li>
              </ul>
            </div>
          </n-collapse-item>

          <n-collapse-item name="disclaimer">
            <template #header>
               <n-flex align="center" size="small">
                <n-icon color="#f0a020"><AlertCircleOutline /></n-icon>
                <span>免责声明</span>
               </n-flex>
            </template>
            <div class="collapse-inner">
              <p class="text-muted text-small">THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.</p>
              <p class="text-muted">本软件基于 MIT 协议开源。开发者不对因使用产生的任何数据丢失或营业额偏差承担责任。基于该软件的二次开发须保留该声明。</p>
              <n-alert type="warning" :show-icon="false">
                <strong>⚠️ 重要提醒</strong>：请养成定时导出数据备份(包括制品信息和营业记录)的习惯，以防止意外发生。
              </n-alert>
            </div>
          </n-collapse-item>
        </n-collapse>
      </section>

      <!-- 7. 底部信息 -->
      <section class="footer-section">
        <n-flex vertical align="center" size="large">
          <div class="dev-info">
            <n-avatar round size="large" color="#eee">R</n-avatar>
            <div class="text-center">
              <div class="author-name">Renko_1055</div>
              <div class="author-title">境界景观学会</div>
            </div>
          </div>
          
          <n-flex>
            <n-button secondary type="default" size="small" @click="copyLink('https://github.com/Academy-of-Boundary-Landscape/ABL-BoothApp', 'GitHub')">
              <template #icon><n-icon><LogoGithub /></n-icon></template> GitHub
            </n-button>
            <n-button secondary type="info" size="small" @click="copyLink('https://abl.secret-sealing.club', '官网')">
              <template #icon><n-icon><GlobeOutline /></n-icon></template> 官网
            </n-button>
            <n-popover trigger="hover">
              <template #trigger>
                <n-button secondary type="success" size="small" @click="copyLink('contact@secret-sealing.club', '邮箱')">
                  <template #icon><n-icon><MailOutline /></n-icon></template> 联系
                </n-button>
              </template>
              <span>contact@secret-sealing.club</span>
            </n-popover>
          </n-flex>
          
          <div class="copyright">
            © 2026 境界景观学会 | Designed for Doujin Circles
          </div>
        </n-flex>

        <!-- 危险操作区 -->
        <div class="danger-zone mt-4">
          <div class="danger-header">
            <n-icon :color="'var(--error-color)'"><TrashOutline /></n-icon> 危险操作
          </div>
          <p class="danger-desc">清空所有数据和图片文件，仅在重置系统时使用。</p>
          <n-button type="error" ghost size="small" @click="resetDatabase">
            重置数据库
          </n-button>
        </div>
      </section>

    </n-card>
  </div>
</template>

<script setup lang="ts">
import { 
  NCard, NAvatar, NTag, NDivider, NGrid, NGridItem, NIcon, NButton, NPopover, 
  NTimeline, NTimelineItem, NFlex, NCollapse, NCollapseItem, NAlert, useMessage, useDialog, NBlockquote
} from 'naive-ui'
import api from '@/services/api'
import { 
  BookOutline, HardwareChipOutline, CodeSlashOutline, DocumentTextOutline,
  StorefrontOutline, LogoGithub, GlobeOutline, MailOutline, LogoWindows,
  LogoAndroid, LogoTux, PhonePortraitOutline, WifiOutline, TrashOutline,
  HeartOutline, ShieldCheckmarkOutline, AlertCircleOutline, GiftOutline 
} from '@vicons/ionicons5'

const message = useMessage()
const dialog = useDialog()

// 数据定义：将原来硬编码在模板里的内容提取出来，使模板更干净
const features = [
  {
    title: '多机联动',
    icon: PhonePortraitOutline,
    color: '#2080f0',
    desc: '只需一台主机，局域网内其他设备扫码同样可运行程序变成“点单机”，低成本实现多设备协同。'
  },
  {
    title: '离线优先',
    icon: WifiOutline,
    color: '#18a058',
    desc: '基于本地 SQLite和局域网部署，无论漫展现场信号如何，记账与库存扣减永远流畅。'
  },
  {
    title: '自动化经营',
    icon: StorefrontOutline,
    color: '#f0a020',
    desc: '自动算总价、弹收款码、生成 Excel 报表。您只需无脑递货，可专注于和同好交流。'
  }
]

const credits = [
  { title: '💻 同人社团', desc: '该项目由同人社团 境界景观学会 开发和维护。' },
  { title: '👥 内测者', desc: '感谢在混乱展会现场忍受 Bug 的内测摊主们。' },
  { title: '🤖 AI 助手', desc: '感谢 Gemini 3.0 Pro 和 Github Copilot 辅助开发。' },
  { title: '🎮 支持者', desc: '感谢东方幻想指南的朋友们对该项目开发的支持。' }
]

// 交互逻辑
const copyLink = async (url: string, label: string) => {
  try {
    await navigator.clipboard.writeText(url)
    message.success(`已复制${label}链接`)
  } catch (err) {
    message.error(`复制失败`)
  }
}

const resetDatabase = () => {
  dialog.error({
    title: '⚠️ 危险操作',
    content: '此操作将不可逆地删除所有展会、商品、订单数据及图片文件。\n确定要完全重置系统吗？',
    positiveText: '确认重置',
    negativeText: '取消',
    onPositiveClick: async () => {
      const load = message.loading('正在重置...')
      try {
        const res = await api.put('/admin/reset-database')
        load.destroy()
        message.success(res.data.message || '重置成功')
        setTimeout(() => {
          sessionStorage.clear()
          window.location.href = '/admin'
        }, 1500)
      } catch (err: any) {
        load.destroy()
        message.error(err.response?.data?.error || '重置失败')
      }
    }
  })
}
</script>

<style scoped>
/* 定义局部变量 */
.about-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 24px 16px;
  --text-primary: var(--n-text-color);
  --text-secondary: var(--n-text-color-3);
  --bg-subtle: rgba(128, 128, 128, 0.08);
}

/* 通用排版 */
.text-muted { color: var(--text-secondary); line-height: 1.6; }
.text-small { font-size: 0.85rem; }
.mb-2 { margin-bottom: 8px; }
.mt-4 { margin-top: 32px; }

/* 头部 Header */
.header-section { text-align: center; padding: 20px 0; }
.logo { margin-bottom: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.1); }
.app-title { margin: 0 0 8px; font-size: 2rem; font-weight: 800; letter-spacing: -0.5px; }
.app-subtitle { font-size: 1.1rem; color: var(--text-secondary); margin: 0 0 24px; }

/* 章节通用 */
.section { margin-bottom: 40px; }
.section-header { display: flex; align-items: center; gap: 10px; margin-bottom: 16px; }
.section-header h2 { margin: 0; font-size: 1.25rem; font-weight: 600; }

/* 核心功能卡片 */
.feature-card { height: 100%; border-radius: 8px; transition: transform 0.2s; }
.feature-card:hover { transform: translateY(-3px); }
.feature-header { margin-bottom: 12px; }
.feature-title { font-weight: 600; font-size: 1.05rem; margin-left: 8px; }
.feature-desc { color: var(--text-secondary); font-size: 0.95rem; margin: 0; line-height: 1.5; }

/* 致谢模块 */
.credit-item {
  background: var(--bg-subtle);
  padding: 12px 16px;
  border-radius: 6px;
  height: 100%;
}
.credit-title { font-weight: 600; font-size: 0.95rem; margin-bottom: 4px; }
.credit-desc { font-size: 0.9rem; color: var(--text-secondary); }

/* 折叠面板内容 */
.collapse-inner { padding: 12px 4px; font-size: 0.95rem; }
.qa-q { font-weight: 600; margin-bottom: 8px; }
.qa-list { margin: 0; padding-left: 20px; color: var(--text-secondary); }
.qa-list li { margin-bottom: 6px; }
.highlight-item { color: var(--n-primary-color); font-weight: 500; }

/* 底部区域 */
.footer-section { margin-top: 48px; text-align: center; }
.dev-info { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
.author-name { font-weight: 700; font-size: 1.1rem; }
.author-title { font-size: 0.85rem; color: var(--text-secondary); }
.copyright { font-size: 0.8rem; color: var(--text-secondary); font-family: monospace; }

/* 危险操作区 */
.danger-zone {
  border: 1px dashed var(--n-error-color);
  background: rgba(208, 48, 80, 0.05);
  border-radius: 8px;
  padding: 16px;
  width: 100%;
  max-width: 400px;
  margin: 0 auto;
}
.danger-header { 
  color: var(--n-error-color); font-weight: 600; display: flex; 
  align-items: center; justify-content: center; gap: 6px; margin-bottom: 4px;
}
.danger-desc { font-size: 0.85rem; color: var(--text-secondary); margin: 0 0 12px; }

/* 移动端微调：利用 CSS 变量微调间距，而非重写整个布局 */
@media (max-width: 600px) {
  .about-container { padding: 12px; }
  .app-title { font-size: 1.6rem; }
  .logo { width: 80px !important; height: 80px !important; }
}
</style>