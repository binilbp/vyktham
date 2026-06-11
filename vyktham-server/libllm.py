#llm helper functions

from dotenv import load_dotenv
load_dotenv()


from langchain.chat_models import init_chat_model
def get_chat_model():
    model = init_chat_model("groq:llama-3.3-70b-versatile")
    return model
