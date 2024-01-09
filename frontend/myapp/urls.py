from django.urls import path
from . import views

urlpatterns = [
    path('', views.home, name='home'),
    path('dashboard', views.dashboard, name='dashboard'),
    path('assistants/<str:assistant_id>', views.assistant_dashboard, name='assistant_dashboard'),
    path('build', views.build, name='build'),
    path('assistants/<str:assistant_id>/delete', views.assistant_delete, name='assistant_delete'),
    path('assistants/<str:assistant_id>/edit', views.assistant_edit, name='assistant_edit'),
]
