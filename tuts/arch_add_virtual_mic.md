# How to add a virtual mic

## Add the virtual speaker

`pactl load-module module-null-sink sink_name=VirtualSpeaker sink_properties=device.description="virtual speaker"`

## Add the virtual mic

this is a loopback deivce
`pactl load-module module-remap-source master=VirtualSpeaker.monitor source_name=VirtualMic source_properties=device.description="virtual mic"`

## Using in OBS

### Add audio mixer mic

In your audio mixer, you should have a `Mic/Aux`. If you dont, figure out how to add it.

### Update the properties of Mic

In your audio mixer, selec the menu button on your mic option. Then Select `Properties`.
A window should pop up in which you will pick the device you use as a microphone. In my case its my `Fifine usb mic`

### Update the advanced audio options

Do the same as the previous step, except go to `Advanced Audio Options`. You will see `Mic/Aux`.
You will also see a table header called `Audio Monitoring`. Go there, and change the dropdown from `Monitor Off` to `Monitor and Output`.

Now close that dialog

### Update OBS advanced audio settings

On the right hand side of the default OBS view, you will see a fiew options:

-   Start Streaming
-   Start Recording
-   Studio mode
-   Settings
-   Exit

Choose `Settings`. A dialog will appear.
Navigate to the `Audio` tab on the left hand side.
Scroll down until you see a section called `Advanced`.

Change the `Monitoring Device` option to be the new virtual device. Should be called `virtual`
Click apply; close.

### Fin

Now you should be able to go into something like Discord and select your input device as the new virtual device.

---

## > [!NOTE]

> If its still not working, open up `Volume Control` app and under the `Recording` nav tab, your `virtual input from` device should be set to `Monitor of virtual`.
