<template>
  <n-modal
    :show="show"
    :mask-closable="true"
    @update:show="val => { if (!val) $emit('close') }"
  >
    <n-card
      :bordered="true"
      size="medium"
      class="payment-card"
      :segmented="{ content: true, footer: true }"
      :style="cardStyle"
    >
      <template #header>
        <div class="header-line">
          请扫码支付 <strong>¥{{ total.toFixed(2) }}</strong>
        </div>
      </template>

      <div class="content-area">
        <div v-if="qrCodeUrl" class="qr-code-container">
          <!-- 注意：不要再给 n-image 外层强行 aspect-ratio/height:100% -->
          <n-image
            :src="qrCodeUrl"
            alt="收款码"
            class="qr-code"
            preview-disabled
            object-fit="contain"
          />
        </div>

        <div v-else class="no-qr-code">
          <p>暂无收款码</p>
          <p>请联系摊主设置收款码</p>
        </div>

        <p class="scan-tip">
          手机浏览器用户请长按二维码图片保存后，<br />
          用微信/支付宝“扫一扫”识别付款
        </p>
        <p class="after-tip">支付完成后，请等待摊主为您配货</p>
      </div>

      <template #footer>
        <n-button block type="primary" @click="$emit('close')">关闭</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup>
import { computed } from 'vue'
import { NModal, NCard, NImage, NButton } from 'naive-ui'

defineEmits(['close'])
const props = defineProps({
  show: { type: Boolean, required: true },
  total: { type: Number, required: true },
  qrCodeUrl: { type: String, default: null }
})

// 响应式卡片宽度：桌面 520px，移动端占 92vw
const cardStyle = computed(() => ({
  width: 'min(520px, 92vw)'
}))
</script>

<style scoped>
.header-line {
  font-size: 1.2rem;
}
.header-line strong {
  color: var(--accent-color);
}

.payment-card {
  text-align: center;
}

/* 让整体内容更容易“一屏放下” */
.content-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.9rem;
  max-height: none;
  overflow: visible;
}

/* 容器只负责居中 + 限制最大占屏高度 */
.qr-code-container {
  width: 100%;
  display: flex;
  justify-content: center;
  /* 防止二维码太高把按钮挤出屏幕：你可以把 60vh 调成 55/65 */
  max-height: 70vh;
}

/*
  关键：不要固定 height，也不要固定正方形比例。
  只给“最大宽度”和“最大高度”，让图片保持原比例等比缩放。
*/
.qr-code {
  display: block;
  width: auto;
  height: auto;

  /* 横向自适应：不超过卡片内容宽度 */
  max-width: 100%;

  /* 桌面端给一个更舒服的上限（可选） */
  max-inline-size: 420px;

  /* 纵向控制：不超过容器(70vh)，以保证整体更容易一屏放下 */
  max-height: 70vh;

  border-radius: 8px;
}

/* n-image 内部 img 的真实控制点 */
:deep(.qr-code img) {
  display: block;
  width: 100%;
  height: auto;              /* 关键：别让它被拉成固定高度 */
  object-fit: contain;       /* 完整展示，不裁切 */
  image-rendering: auto;
}

.no-qr-code {
  padding: 2rem;
  color: var(--text-disabled);
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-color);
}

.no-qr-code p {
  margin: 0.5rem 0;
}

.payment-card p {
  margin: 0;
  color: var(--text-muted);
}

.after-tip {
  margin-top: -0.25rem;
}

.scan-tip {
  color: var(--accent-color);
  font-size: 1.05rem;
  margin: 0.4rem 0 0.4rem 0;
  font-weight: bold;
  line-height: 1.5;
}

/* 移动端：把“桌面 420px 上限”去掉，让它真正跟随屏幕宽度 */
@media (max-width: 600px) {
  .payment-card {
    border-radius: 8px;
  }

  .qr-code-container {
    width: 100%;
    margin: 0.25rem 0 0.6rem 0;
    max-height: 65vh; /* 移动端稍微更紧一点，避免挤出按钮 */
  }

  .qr-code {
    max-inline-size: 100%;
    max-height: 65vh;
  }
}
</style>
