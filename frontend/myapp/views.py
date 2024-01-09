import requests
from django.shortcuts import render

## VIEWS ##


def home(request):
    return render(request, 'home.html')

def dashboard(request):
    assistant_list = get_assistant_list()

    # Pass the data to the template
    return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'hide_form': True})

def build(request):
    return render(request, 'build.html')

def assistant_dashboard(request, assistant_id):
    assistant_list = get_assistant_list()
    assistant_info = get_assistant(assistant_id)

    # Pass the data to the template
    return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'assistant_info': assistant_info})

def assistant_delete(request, assistant_id):
    if request.method == 'GET':
        assistant_info = get_assistant(assistant_id)
        return render(request, 'confirm_delete.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        # Make a DELETE request to the specified URL
        delete_response = delete_assistant(assistant_id)

        # Redirect to the dashboard
        return dashboard(request)
    
def assistant_edit(request, assistant_id):
    if request.method == 'GET':
        assistant_info = get_assistant(assistant_id)
        return render(request, 'build.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        # Make a PUT request to the specified URL
        response = requests.put('http://localhost:8080/assistants/' + assistant_id, json=request.POST)

        # Redirect to the dashboard
        return dashboard(request)
    

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

def delete_assistant(assistant_id):
    # Make a DELETE request to the specified URL
    response = requests.delete('http://localhost:8080/assistants/' + assistant_id)

    # Parse the response as JSON
    delete_response = response.json()

    return delete_response