---
Created: 2023-02-28T20:00
tags:
  - DevOps
---
- Install
    
    ```Shell
    apt install ansible
    ```
    
> [!info] /etc/ansible/ansible.cfg debian 11 好像不會有可以自己建立
    

- 查詢
    
    ```Shell
    ansible-doc -l
    ```
    
    
    ```Shell
    ansible-doc <模組名稱>
    ```
    
    
    - example
        
        ```Shell
        ansible-doc services
        ```
        
        ![[Assets/Note/Tech/Ansible 基本操作安裝/01-example.png|01-example.png]]
        
    

- Inventory
    
    可以使用ini格式或是yaml格式寫
    
    - static inventory
        
        ```Plain
        192.168.10.2
        
        [all:vars]
        ansible_user-administrator
        ansible_ssh_pass=Skills39
        ansible_sudo_pass=Skills39
        ```
        
        > [!important] 此為ini 官網很多這個
        
    - dynamic inventory
    - 執行參數i
        
        ```Shell
        ansible all -i <static inventory path> -m ping
        ```
        
        ![[Assets/Note/Tech/Ansible 基本操作安裝/02-執行參數i.png|02-執行參數i.png]]


- Playbook
    - 完全沒有用的playbook
        
        ```YAML
        ---
        - name: Playbook
        	hosts: all
          tasks: 
        ```
        
    - 安裝nginx 並啟用
        
        - site.yml
            
            ```YAML
            ---
            - name: Playbook
            	hosts: all
            	tasks:
            		- name: install Nginx
            			become: yes
            			apt:
            				name: nginx
            				state: present
            
            		- name: enable Nginx service
            			ansible.builtin.service:
            			name: nginx
            			enabled: true
            			state: started
            ```
            
        - 執行playbook
            
            ```YAML
            ansible-playbook -i hosts site.yml
            ```
            

- 常用Moudel
    
    - apt
    - service
    - file