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
    Render the specified assistant's dashboard populated with the assistant's info on GET requests.
    Query the assistant with the user's input and display the assistant's response on POST request.
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
    Render the assistant creation page on GET requests.
    Create a new assistant on POST requests.
    """
    if request.method == 'GET':
        return render(request, 'build.html')
    if request.method == 'POST':
        form_data = request.POST
        response = create_assistant(form_data)
        return dashboard(request)

def assistant_edit(request, assistant_id):
    """
    Render the assistant edit page on GET requests.
    Edit the assistant on POST requests.
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
    Render the assistant deletion page on GET requests.
    Delete the assistant on POST requests.
    """
    if request.method == 'GET':
        assistant_info = get_assistant_info(assistant_id)
        return render(request, 'confirm_delete.html', {'assistant_info': assistant_info})
    elif request.method == 'POST':
        delete_response = delete_assistant(assistant_id)
        return dashboard(request)
    
## HELPER FUNCTIONS ##

def get_assistant_list():
    """
    Requests the list of assistants from the backend.
    """
    response = requests.get('http://backend:8080/assistants')
    assistant_list = response.json()
    return assistant_list

def get_assistant_info(assistant_id):
    """
    Requests assistant info from the backend.
    """
    response = requests.get('http://backend:8080/assistants/' + assistant_id)
    assistant_info = response.json()
    return assistant_info

def create_assistant(form_data):
    """
    Requests the creation of a new assistant from the backend.
    """
    response = requests.post('http://backend:8080/assistants', json=form_data)
    create_response = response.json()
    return create_response

def edit_assistant(assistant_id, form_data):
    """
    Requests the editing of an assistant from the backend.
    """
    response = requests.put('http://backend:8080/assistants/' + assistant_id, json=form_data)
    edit_response = response.json()
    return edit_response

def delete_assistant(assistant_id):
    """
    Requests the deletion of an assistant from the backend.
    """
    response = requests.delete('http://backend:8080/assistants/' + assistant_id)
    delete_response = response.json()
    return delete_response

def create_thread():
    """
    Requests the creation of a new thread from the backend.
    """
    response = requests.post('http://backend:8080/threads')
    thread_id = response.json()
    return thread_id.get('thread_id')

def query_assistant(assistant_id, form_data):
    """
    Requests query response from the backend.
    """
    query_response = requests.post('http://backend:8080/assistants/' + assistant_id, json=form_data)
    messages_response = requests.get('http://backend:8080/threads/' + form_data['thread_id'])
    messages = messages_response.json()
    return messages