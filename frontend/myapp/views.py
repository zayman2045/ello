import requests
from django.shortcuts import render

## VIEWS ##

# Render the home page
def home(request):
    return render(request, 'home.html')

# Render the dashboard populated with the list of assistants
def dashboard(request):
    assistant_list = get_assistant_list()
    return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'hide_form': True})

# Render the specified assistant's dashboard populated with the assistant's info
def assistant_dashboard(request, assistant_id):
    assistant_list = get_assistant_list()
    assistant_info = get_assistant(assistant_id)
    return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'assistant_info': assistant_info})

# Render the assistant creation page and handle the creation of a new assistant
def assistant_create(request):
    if request.method == 'GET':
        return render(request, 'build.html')
    if request.method == 'POST':
        form_data = request.POST
        response = create_assistant(form_data)
        return dashboard(request)

# Render the assistant deletion page and handle the deletion of an assistant
def assistant_delete(request, assistant_id):
    if request.method == 'GET':
        assistant_info = get_assistant(assistant_id)
        return render(request, 'confirm_delete.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        delete_response = delete_assistant(assistant_id)
        return dashboard(request)

# Render the assistant edit page and handle the editing of an assistant
def assistant_edit(request, assistant_id):
    if request.method == 'GET':
        assistant_info = get_assistant(assistant_id)
        return render(request, 'build.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        form_data = request.POST
        edit_response = edit_assistant(assistant_id, form_data)
        return dashboard(request)
    
    
## HELPER FUNCTIONS ##

# Requests the list of assistants from the backend
def get_assistant_list():
    # Make a GET request to the specified URL
    response = requests.get('http://localhost:8080/assistants')

    # Parse the response as JSON and return the list of assistants
    assistant_list = response.json()
    return assistant_list

# Requests assistant info from the backend
def get_assistant(assistant_id):
    # Make a GET request to the specified URL
    response = requests.get('http://localhost:8080/assistants/' + assistant_id)

    # Parse the response as JSON and return the assistant info
    assistant_info = response.json()
    return assistant_info

# Requests the creation of a new assistant from the backend
def create_assistant(form_data):
    # Make a POST request to the specified URL
    response = requests.post('http://localhost:8080/assistants', json=form_data)

    # Parse the response as JSON and return the response
    create_response = response.json()
    return create_response

# Requests the editing of an assistant from the backend
def edit_assistant(assistant_id, form_data):
    # Make a PUT request to the specified URL
    response = requests.put('http://localhost:8080/assistants/' + assistant_id, json=form_data)

    # Parse the response as JSON and return the response
    edit_response = response.json()
    return edit_response

# Requests the deletion of an assistant from the backend
def delete_assistant(assistant_id):
    # Make a DELETE request to the specified URL
    response = requests.delete('http://localhost:8080/assistants/' + assistant_id)

    # Parse the response as JSON and return the response
    delete_response = response.json()
    return delete_response