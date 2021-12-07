#
# Date: 06.12.2021  dd/mm/yyyy
# Title: A beginners journey into the world of Radix and Scrypto - Day 1
#
# Description
# I've never coded in my life before. Today I set up the working environment to get started with Scrypto (I hope). I have some years of ICT experience though.
# 
# 1st decision:
# a) Start this journey on Windows
# b) Start this journey on Linux
#
# The Radix/Scrypto community in Discord suggested that I'll likely run into less problems if I opt for Linux from the very beginning.
# The core Radix team does their Scrypto magic with Linux and it's expected that a bigger part of the developers will work on Linux (or Mac) as well, thus it will be easier to get help from the community now and in the future. I'll happily take any help I can get before ragequitting, so Linux it is.
#
#
# 2nd decision:
# a) Damn, they said Linux. Should I buy a new notebook for development? (Set up everything from scratch? Run into driver issues? Which notebook..? Who has time for that??)
# b) Or can I set it all up in the cloud and use my machine to remotely work on it? (I'm lazy and this seemed like the fastest path)
#
# I opted for the second option. It was fast. Price okay I guess. First two months currently are free for trial - I'll take it.
# With Digitalocean an Ubuntu machine was set up in no time (Other options such as AWS free tier, Linode, GCP and others are available as well - I just haven't tested them). While creating your machine on Digitalocean, follow this guide here -> https://github.com/cdr/deploy-code-server/blob/main/guides/digitalocean.md
# Later on this will allow you to launch a development environment in the browser which is pretty epic - I'm currently typing this very file on it. No need to install anything on your local machine (it seems). Pretty cool.
#
# After the first start of your Ubuntu machine, you'll need to install "gcc". You can launch a console window of your Ubuntu machine in the browser on your Digitalocean dashboard. Once you're connected, do this:
# $ sudo apt-get update
# $ sudo apt install build-essential
#
# Once everything is installed, check if the installation was successful:
# $ gcc --version
# This should print something on your screen.
# 
# If everything worked, proceed with the installation as it's described here:
# https://www.scrypto-tutorial.com/getting-started/readme
#
#
# I've cloned the radix-scrypto branch by hitting the big green "Code" button here:
# https://github.com/radixdlt/radixdlt-scrypto
#
# Initially I only wanted to clone the subdirectory that contains the scrypto examples - TIL: That's not possible. Apparently on github you always clone the entire repo (if that's the right wording)
# Since I don't care for most of the stuff yet, I've proceeded to delete almost everything except the examples and .github folder (which seems important, I'm just guessing though - Edit: This turned out to be a mistake! I think I have to delete this folder too, otherwise it won't let me copy the project to my repository) and I created a new folder "helloworld" in which I will save this very file here.
#
#
# Next I'll check if I can get all of this uploaded to my github errr... account (?) or whatever it is called.
# I think the workflow for doing exactly that is explained here https://www.youtube.com/watch?v=S7TbHDN8EXA "How To Use GitHub with VS Code in 2020 | Merge, Branch & Pull Request | Part 5"
# Then I'll call it a day. (Ok, getting this file up into my github repo (?) has been the trickiest part of everything, still haven't quite figured it out yet...)
# 
# =======================================
# Additional resources:
# https://www.ssh.com/academy/ssh/putty/windows/puttygen - I didn't need it just yet, but might be handy at some point
# https://code.visualstudio.com/docs/remote/remote-overview - I'm not sure if we're doing exactly this but the architecture kind of looks like what I'm doing here with this code-server thing
# https://www.youtube.com/watch?v=9Emn2YQNDl0 - VSCode In The Cloud - Setup a Remote Dev Environment
# ========================================
#
# I am Guybrush Threepwood and I want to become a mighty Scrypto developer.
# ========================================
#
# Edit:
# Sleeped over the whole thing. I didn't succeed yesterday. I lack the git basics. The simplest things are still hard. How to interact with github?
# Biggest challenge is/was to copy the things from radixdlt-scrypto, modify them and then upload it to MY github repo (?). I failed at that.
# I think I have figured out why now though. The second video mentions that the ".git" folder needs to be deleted, so the whole thing isn't linked to Radix' Github anymore
# https://www.youtube.com/watch?v=3Tn58KQvWtU - How To Use GitHub with VS Code in 2020 | Commit & Push | Part 1
# https://www.youtube.com/watch?v=sz2EM-gkEs0 - How To Use GitHub with VS Code in 2020 | Clone | Part 2
# ========================================