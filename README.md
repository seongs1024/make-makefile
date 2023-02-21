# make-makefile

The simple Makefile generator

```shell
Usage: mm [OPTIONS]

Options:
  -n, --name <NAME>  Name of the project
  -k, --kind <KIND>  Binary or library [default: bin] [possible values: bin, lib]
  -h, --help         Print help
  -V, --version      Print version
```

### Generate a template

The `mm` generates `Makefile` template in your project directory.

You can change `{PROGRAM}` to your project name.

```shell
mm 
```

```Makefile
#The Target
#For Binary: <name>, For Library: lib<name>.a
NAME		:=	{PROGRAM}

#Source Files
SOURCES		:=	main.c \

#The Directories, Source, Includes, Dependencies
TESTDIR		:=	test
BONUSDIR	:=	bonus
SRCDIR		:=	src
INCDIR		:=	inc
DEPDIR		:=	dep
BLDDIR		:=	obj

#Dependencies
DEPLIBS		:=	
DEPDIRS		:=	

#Compiler, Linker, Flags
CC			:=	cc
# DEBUG		:=	-g
# SANITIZE	:=	-fsanitize=address
CFLAGS		:=	-Wall -Wextra -Werror

#-------------------------------------------------------------------------------
#DO NOT EDIT BELOW THIS LINE
#-------------------------------------------------------------------------------
TARGET		:=	$(suffix $(NAME))
CEXT		:=	c
OEXT		:=	o
INC			:=	-I$(INCDIR) $(addprefix -I$(DEPDIR)/, $(DEPDIRS))
LIB			:=	$(addprefix -l, $(DEPLIBS))
LID			:=	$(addprefix -L$(DEPDIR)/, $(DEPDIRS))
SRCS		:=	$(addprefix $(SRCDIR)/,$(SOURCES))
OBJS		:=	$(patsubst $(SRCDIR)/%,$(BLDDIR)/%,$(SRCS:.$(CEXT)=.$(OEXT)))

#Defauilt Make
all: $(NAME)

bonus: 
	@make -C $(BONUSDIR)
	@cp $(BONUSDIR)/$(NAME) ./

#Remake
re: fclean all

#Make the Directories
directories:
	@mkdir -p $(BLDDIR)

#Dependencies
depend:
	@for ddir in $(addprefix $(DEPDIR)/, $(DEPDIRS)); do \
		make -s -C $$ddir; \
	done

#Clean only Objects
clean:
	@for ddir in $(addprefix $(DEPDIR)/, $(DEPDIRS)); do \
		make -s -C $$ddir clean; \
	done
	@$(RM) -rf $(BLDDIR)

#Full Clean, Objects and Binaries
fclean: clean
	@for ddir in $(addprefix $(DEPDIR)/, $(DEPDIRS)); do \
		make -s -C $$ddir fclean; \
	done
	@$(RM) -rf $(NAME)

#Link
$(NAME): $(OBJS)
	@for ddir in $(addprefix $(DEPDIR)/, $(DEPDIRS)); do \
		make -s -C $$ddir; \
	done
ifeq ($(TARGET), .a)
	$(AR) -rc $@ $^
else
	$(CC) $(SANITIZE) $(DEBUG) $(INC) -o $(NAME) $^ $(LID) $(LIB)
endif

#Compile
$(BLDDIR)/%.$(OEXT): $(SRCDIR)/%.$(CEXT) 
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(SANITIZE) $(DEBUG) $(INC) -c -o $@ $<

#Non-File Targets
.PHONY: all re clean fclean bonus directories depend
```

#### Name specifier

```shell
mm --name hello
```

```Makefile
#The Target
#For Binary: <name>, For Library: lib<name>.a
NAME		:=	hello
```

#### Kind option

##### Binary template

```shell
mm --name hello --kind=bin
```

```Makefile
#The Target
#For Binary: <name>, For Library: lib<name>.a
NAME		:=	hello
```

##### Library template

```shell
mm --name hello --kind=lib
```

```Makefile
#The Target
#For Binary: <name>, For Library: lib<name>.a
NAME		:=	libhello.a
```