import json
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

from libllm import get_chat_model


chat_model = get_chat_model()
router = APIRouter()


class FormatRequest(BaseModel):
    markdown: str

class FormatResponse(BaseModel):
    markdown: str



SYSTEM_PROMPT = """
Role: You are a TipTap Markdown formatter specialist
Job: format the given tiptap markdown so that it becomes a better data for notekeeping

Output Format: Your response MUST be valid, minified JSON matching exactly this structure. 
Crucial: Do NOT include literal newlines in the JSON string values. All newlines inside the markdown text MUST be explicitly escaped as '\\n'.

{
    "reply": "your reply to the user goes here if any",
    "markdown": "the formatted markdown content here"
}
"""



@router.post("/format_markdown")
async def extract_profile(request: FormatRequest ) -> dict:
    print("> Received user text format request")
    user_markdown = request.markdown
    
    messages = [
            {"role": "system", "content": SYSTEM_PROMPT },
            {"role": "user", "content": user_markdown }
    ]    

    print("> Invoking model...")
    try:
        ai_result = chat_model.invoke(messages)
        ai_result = ai_result.content.strip()
        print(f"> Recieved respone: {ai_result}")
        parsed_data = json.loads( ai_result )
        ai_markdown = parsed_data.get("markdown", "")

        print(f"> AI Markdown: {ai_markdown}")

        return {
            "markdown": ai_markdown
        }

    except json.JSONDecodeError as e:
        print(f"!! Failed to parse AI JSON: {ai_result}")
        raise HTTPException(status_code=500, detail="AI returned invalid JSON structure.")

    except Exception as e:
        print(f"!! Error: {e}")
        raise HTTPException(status_code=500, detail=str(e))



