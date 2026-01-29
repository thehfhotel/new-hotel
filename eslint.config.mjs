import nextConfig from 'eslint-config-next'
import coreWebVitals from 'eslint-config-next/core-web-vitals'

const eslintConfig = [
  ...nextConfig,
  ...coreWebVitals,
  {
    ignores: ['node_modules/', '.next/', 'thai-id-middleware/', 'thai-id-middleware-tauri/'],
  },
]

export default eslintConfig
