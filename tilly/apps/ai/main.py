from fastapi import FastAPI
from pydantic import BaseModel, Field

app = FastAPI(title="TILLY AI Service", version="0.1.0")


class AssessmentRequest(BaseModel):
    skill: str = Field(min_length=2)
    answers: list[int] = Field(min_length=1)
    completion_seconds: int = Field(gt=0)


class RecommendationOpportunity(BaseModel):
    id: str
    title: str
    location: str
    required_skills: list[str]


class RecommendationRequest(BaseModel):
    user_skills: list[str] = Field(min_length=1)
    opportunities: list[RecommendationOpportunity] = Field(min_length=1)


class TrustScoreRequest(BaseModel):
    completed_jobs: int = Field(ge=0)
    payment_reliability: float = Field(ge=0, le=100)
    customer_reviews: float = Field(ge=0, le=100)
    response_speed: float = Field(ge=0, le=100)


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok", "service": "tilly-ai"}


@app.post("/assessments/score")
def score_assessment(payload: AssessmentRequest) -> dict:
    average_answer = sum(payload.answers) / len(payload.answers)
    speed_bonus = 10 if payload.completion_seconds <= 300 else 0
    score = min(100.0, average_answer + speed_bonus)

    return {
        "skill": payload.skill,
        "score": round(score, 2),
        "passed": score >= 60,
        "badge": f"{payload.skill.title()} Verified" if score >= 60 else None,
    }


@app.post("/recommendations")
def recommend(payload: RecommendationRequest) -> dict:
    user_skill_set = {skill.lower() for skill in payload.user_skills}
    scored = []

    for opportunity in payload.opportunities:
        required = {skill.lower() for skill in opportunity.required_skills}
        overlap = len(user_skill_set.intersection(required))
        score = (overlap / max(len(required), 1)) * 100
        scored.append({
            "id": opportunity.id,
            "title": opportunity.title,
            "location": opportunity.location,
            "semantic_match": round(score, 2),
        })

    scored.sort(key=lambda item: item["semantic_match"], reverse=True)

    return {"matches": scored[:3]}


@app.post("/trust-score")
def trust_score(payload: TrustScoreRequest) -> dict[str, float]:
    completed_jobs_score = min(payload.completed_jobs, 20) / 20 * 100
    score = (
        0.40 * completed_jobs_score
        + 0.25 * payload.payment_reliability
        + 0.20 * payload.customer_reviews
        + 0.15 * payload.response_speed
    )
    return {"trust_score": round(score, 2)}
