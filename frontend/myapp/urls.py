from django.urls import path
from . import views

urlpatterns = [
    path('', views.home, name='home'),
    path('assistants', views.dashboard, name='dashboard'),
    path('assistants/create', views.assistant_create, name='assistant_create'),
    path('assistants/<str:assistant_id>', views.assistant_dashboard, name='assistant_dashboard'),
    path('assistants/<str:assistant_id>/delete', views.assistant_delete, name='assistant_delete'),
    path('assistants/<str:assistant_id>/edit', views.assistant_edit, name='assistant_edit'),
]