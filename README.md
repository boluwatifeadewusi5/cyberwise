# TILLY – AI-Powered Opportunity Engine for Youth

Tagline: **"Your skills are your CV."**

This monorepo contains a hackathon MVP with:
- `tilly/apps/web` → Next.js mobile-first frontend
- `tilly/apps/api` → Rust (Axum) API for users, gigs, trust score, and payments
- `tilly/apps/ai` → Python FastAPI AI microservice for scoring/recommendations

## 1) Prerequisites

Install these first:
- **Node.js 20+** and npm
- **Rust** (stable) + Cargo
- **Python 3.11+**
- **Git**

Optional but recommended:
- **tsx** (for running TypeScript scripts):
  ```bash
  npm install -g tsx
  # or project-local
  npm install --save-dev tsx
  ```

For Android APK builds:
- **Android Studio**
- **Java 17+**
- Android SDK + emulator/device setup

## 2) Environment Variables

### API (`tilly/apps/api`)
Create `tilly/apps/api/.env`:

```env
# Gemini trust-score validation
GEMINI_API_KEY=your_gemini_api_key

# Squad payout integration
SQUAD_BASE_URL=https://api-d.squadco.com
SQUAD_SECRET_KEY=your_squad_secret_key
SQUAD_BUSINESS_ID=your_squad_business_id
```

> If `GEMINI_API_KEY` or `SQUAD_SECRET_KEY` is missing, the API safely falls back to local scoring/simulated payment status.

### Web (`tilly/apps/web`)
Create `tilly/apps/web/.env.local`:

```env
NEXT_PUBLIC_API_URL=http://localhost:8080
```

## 3) Install Dependencies

```bash
# Web
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/web && npm install

# API
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/api && cargo fetch

# AI
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/ai && pip install -r requirements.txt
```

## 4) Run Locally

Open 3 terminals.

### Terminal A: Rust API
```bash
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/api
cargo run
```
API runs on `http://localhost:8080`

### Terminal B: AI service
```bash
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/ai
uvicorn main:app --reload --port 8000
```
AI runs on `http://localhost:8000`

### Terminal C: Web app
```bash
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/web
npm run dev
```
Web runs on `http://localhost:3000`

## 5) Build & Validation Commands

```bash
# Web
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/web
npm run lint && npm run build

# API
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/api
cargo test && cargo build

# AI
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/ai
python -m py_compile main.py
```

## 6) Key API Endpoints

### API service (`:8080`)
- `GET /health`
- `POST /users`
- `GET /users`
- `GET /users/{id}/trust-score` (Gemini-backed validation + fallback)
- `POST /hustles`
- `GET /hustles?location=ikeja`
- `POST /hustles/{id}/complete` (Squad payout attempt + transaction record)
- `GET /transactions?user_id={id}`

### AI service (`:8000`)
- `GET /health`
- `POST /assessments/score`
- `POST /recommendations`
- `POST /trust-score`

## 7) Deployment (Hackathon-friendly)

### Frontend (Vercel)
1. Push repo to GitHub
2. Import `tilly/apps/web` as project root
3. Set `NEXT_PUBLIC_API_URL` to deployed API URL
4. Deploy

### API + AI (Railway)
Create two Railway services:
- Service 1 root: `tilly/apps/api` (Rust)
- Service 2 root: `tilly/apps/ai` (Python)

Set the environment variables shown above for API service.

## 8) Android APK Build Guide (mobile app wrapper)

Use Capacitor to wrap the Next.js export into Android.

### A. Build web assets
```bash
cd /home/runner/work/cyberwise/cyberwise/tilly/apps/web
npm install
npm run build
```

### B. Add Capacitor packages
```bash
npm install @capacitor/core @capacitor/android
npm install -D @capacitor/cli tsx
```

### C. Initialize Capacitor
```bash
npx cap init tilly com.tilly.mobile --web-dir=.next
```

If you prefer static export output, configure Next export and point `--web-dir` to `out`.

### D. Add Android platform + sync
```bash
npx cap add android
npx cap sync android
```

### E. Open Android Studio + build APK
```bash
npx cap open android
```
In Android Studio:
- **Build → Build Bundle(s)/APK(s) → Build APK(s)**
- APK output is shown by Android Studio after build completes.

## 9) Notes

- UI is intentionally mobile-first.
- Gemini and Squad integrations are environment-driven and safe by default.
- Replace placeholder payout beneficiary data with real account details before production use.
