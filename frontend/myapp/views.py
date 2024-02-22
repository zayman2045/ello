import requests
from django.shortcuts import render

## VIEWS ##

def home(request):
    """
    Render the home page.
    """
    return render(request, 'home.html')

def dashboard(request):
    """
    Render the dashboard populated with the list of assistants.
    """
    assistant_list = get_assistant_list()
    return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'hide_form': True})

def assistant_dashboard(request, assistant_id):
    """
    Render the specified assistant's dashboard populated with the assistant's info.
    """
    if request.method == 'GET':
        assistant_list = get_assistant_list()
        assistant_info = get_assistant_info(assistant_id)
        thread_id = create_thread()
        return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'assistant_info': assistant_info, 'thread_id': thread_id})
    elif request.method == 'POST':
        assistant_list = get_assistant_list()
        assistant_info = get_assistant_info(assistant_id)
        form_data = request.POST
        messages = query_assistant(assistant_id, form_data)
        return render(request, 'dashboard.html', {'assistant_list': assistant_list, 'assistant_info': assistant_info, 'thread_id': request.POST['thread_id'], 'messages': messages})

def assistant_create(request):
    """
    Render the assistant creation page and handle the creation of a new assistant.
    """
    if request.method == 'GET':
        return render(request, 'build.html')
    if request.method == 'POST':
        form_data = request.POST
        response = create_assistant(form_data)
        return dashboard(request)

def assistant_edit(request, assistant_id):
    """
    Render the assistant edit page and handle the editing of an assistant.
    """
    if request.method == 'GET':
        assistant_info = get_assistant_info(assistant_id)
        return render(request, 'build.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        form_data = request.POST
        edit_response = edit_assistant(assistant_id, form_data)
        return dashboard(request)

def assistant_delete(request, assistant_id):
    """
    Render the assistant deletion page and handle the deletion of an assistant.
    """
    if request.method == 'GET':
        assistant_info = get_assistant_info(assistant_id)
        return render(request, 'confirm_delete.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        delete_response = delete_assistant(assistant_id)
        return dashboard(request)
    
## HELPER FUNCTIONS ##

# Requests the list of assistants from the backend
def get_assistant_list():
    # Make a GET request to the specified URL
    response = requests.get('http://backend:8080/assistants')

    # Parse the response as JSON and return the list of assistants
    assistant_list = response.json()
    return assistant_list

# Requests assistant info from the backend
def get_assistant_info(assistant_id):
    # Make a GET request to the specified URL
    response = requests.get('http://backend:8080/assistants/' + assistant_id)

    # Parse the response as JSON and return the assistant info
    assistant_info = response.json()
    return assistant_info

# Requests the creation of a new assistant from the backend
def create_assistant(form_data):
    # Make a POST request to the specified URL
    response = requests.post('http://backend:8080/assistants', json=form_data)

    # Parse the response as JSON and return the response
    create_response = response.json()
    return create_response

# Requests the editing of an assistant from the backend
def edit_assistant(assistant_id, form_data):
    # Make a PUT request to the specified URL
    response = requests.put('http://backend:8080/assistants/' + assistant_id, json=form_data)

    # Parse the response as JSON and return the response
    edit_response = response.json()
    return edit_response

# Requests the deletion of an assistant from the backend
def delete_assistant(assistant_id):
    # Make a DELETE request to the specified URL
    response = requests.delete('http://backend:8080/assistants/' + assistant_id)

    # Parse the response as JSON and return the response
    delete_response = response.json()
    return delete_response

# Requests the creation of a new thread from the backend
def create_thread():
    # Make a POST request to the specified URL
    response = requests.post('http://backend:8080/threads')

    # Parse the response as JSON and return the response
    thread_id = response.json()
    return thread_id.get('thread_id')

# Requests query response from the backend
def query_assistant(assistant_id, form_data):
    # Make a POST request to the specified URL
    query_response = requests.post('http://backend:8080/assistants/' + assistant_id, json=form_data)

    # Make a GET request to the specified URL
    messages_response = requests.get('http://backend:8080/threads/' + form_data['thread_id'])

    # Parse the response as JSON and return the response
    messages = messages_response.json()
    return messages
