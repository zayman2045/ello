import requests
from django.shortcuts import render

## VIEWS ##


def home(request):
    return render(request, 'home.html')

def dashboard(request):
    assistant_list = get_assistant_list()

    # Pass the data to the template
    return render(request, 'dashboard.html', {'assistant_list': assistant_list})

def build(request):
    return render(request, 'build.html')

def assistant_dashboard(request, assistant_id):
    assistant_list = get_assistant_list()
    assistant_info = get_assistant(assistant_id)

    # Pass the data to the template
    return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'assistant_info': assistant_info})


## HELPER FUNCTIONS ##

# Requests the list of assistants from the backend
def get_assistant_list():
    # Make a GET request to the specified URL
    response = requests.get('http://localhost:8080/assistants')

    # Parse the response as JSON
    assistant_list = response.json()

    return assistant_list

# Requests assistant info from the backend
def get_assistant(assistant_id):
    # Make a GET request to the specified URL
    response = requests.get('http://localhost:8080/assistants/' + assistant_id)

    # Parse the response as JSON
    assistant_info = response.json()

    return assistant_info