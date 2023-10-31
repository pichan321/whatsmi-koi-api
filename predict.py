# Check accuracy
from keras.utils import load_img, img_to_array
import numpy as np
from keras.models import load_model
import tensorflow as tf
from type import get_types

# gpu_devices = tf.config.experimental.list_physical_devices('GPU')
# for device in gpu_devices:
#     tf.config.experimental.set_memory_growth(device, True)
# tf.config.experimental.set_visible_devices(gpu_devices[0], 'GPU')
import sys
filename = sys.argv[1]

predict = ['Asagi','Bekko','Doitsu Koi','Ghosiki','Goromo','Hikarimoyo','Hikarimuji mono','Hikariutsuri','Kanoko Koi','Kawarimono','Kin/Ginrin','Kohaku','Sanke','Showa','Shusui','Tancho','Utsuri','Yamato Nishiki']
predict = get_types()
predict = np.array(predict)

# gpu_devices = tf.config.experimental.list_physical_devices('GPU')
# for device in gpu_devices:
#     tf.config.experimental.set_memory_growth(device, True)
# tf.config.experimental.set_visible_devices(gpu_devices[0], 'GPU')


# with tf.device('/GPU:0'):
model_ANN = load_model("transferred-classification-final")
img_width, img_height = 224, 224


img = load_img(filename,target_size=(img_width, img_height))
img = img_to_array(img)
img = img.reshape(1,img_width,img_height,3)
img = img.astype('float32')
img = img/255
# print(model_ANN.predict(img))
result = np.argmax(model_ANN.predict(img),axis=1)

print(result)


types = get_types()
print(types)

print("Predicted", types[result[0]])
# print(model_ANN.predict(img))
# print(predict[result])

# tf.keras.backend.clear_session()
